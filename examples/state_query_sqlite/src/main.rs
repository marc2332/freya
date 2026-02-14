#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{
    Arc,
    Mutex,
};

use freya::{
    prelude::*,
    query::*,
};
use rusqlite::Connection;

#[derive(Clone, Debug)]
struct Todo {
    id: i64,
    title: String,
    completed: bool,
}

type Db = Captured<Arc<Mutex<Connection>>>;
type Error = Box<dyn std::error::Error + Send + Sync>;

fn init_db() -> Arc<Mutex<Connection>> {
    let conn =
        Connection::open("./state_query_sqlite_data").expect("Failed to create in-memory database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )
    .expect("Failed to create table");
    Arc::new(Mutex::new(conn))
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct GetTodos;

impl QueryCapability for GetTodos {
    type Ok = Vec<Todo>;
    type Err = Error;
    type Keys = ();

    async fn run(&self, _keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let db = consume_context::<Db>();
        blocking::unblock(move || {
            let conn = db.lock().map_err(|e| -> Error { e.to_string().into() })?;
            let mut stmt = conn.prepare("SELECT id, title, completed FROM todos")?;
            let todos = stmt
                .query_map([], |row| {
                    Ok(Todo {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        completed: row.get(2)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(todos)
        })
        .await
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct AddTodo;

impl MutationCapability for AddTodo {
    type Ok = ();
    type Err = Error;
    type Keys = String;

    async fn run(&self, title: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let db = consume_context::<Db>();
        let title = title.clone();
        blocking::unblock(move || {
            let conn = db.lock().map_err(|e| -> Error { e.to_string().into() })?;
            conn.execute("INSERT INTO todos (title) VALUES (?)", [&title])?;
            Ok(())
        })
        .await
    }

    async fn on_settled(&self, _keys: &Self::Keys, _result: &Result<Self::Ok, Self::Err>) {
        QueriesStorage::<GetTodos>::invalidate_matching(()).await;
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct ToggleTodo(i64);

impl MutationCapability for ToggleTodo {
    type Ok = ();
    type Err = Error;
    type Keys = i64;

    async fn run(&self, _: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let db = consume_context::<Db>();
        let id = self.0;
        blocking::unblock(move || {
            let conn = db.lock().map_err(|e| -> Error { e.to_string().into() })?;
            conn.execute(
                "UPDATE todos SET completed = NOT completed WHERE id = ?",
                [id],
            )?;
            Ok(())
        })
        .await
    }

    async fn on_settled(&self, _keys: &Self::Keys, _result: &Result<Self::Ok, Self::Err>) {
        QueriesStorage::<GetTodos>::invalidate_matching(()).await;
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct DeleteTodo(i64);

impl MutationCapability for DeleteTodo {
    type Ok = ();
    type Err = Error;
    type Keys = i64;

    async fn run(&self, _: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let db = consume_context::<Db>();
        let id = self.0;
        blocking::unblock(move || {
            let conn = db.lock().map_err(|e| -> Error { e.to_string().into() })?;
            conn.execute("DELETE FROM todos WHERE id = ?", [id])?;
            Ok(())
        })
        .await
    }

    async fn on_settled(&self, _keys: &Self::Keys, _result: &Result<Self::Ok, Self::Err>) {
        QueriesStorage::<GetTodos>::invalidate_matching(()).await;
    }
}

#[derive(PartialEq)]
struct TodoRow {
    id: i64,
    title: String,
    completed: bool,
}

impl TodoRow {
    fn new(todo: &Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title.clone(),
            completed: todo.completed,
        }
    }
}

impl Component for TodoRow {
    fn render(&self) -> impl IntoElement {
        let id = self.id;
        let toggle_mutation = use_mutation(Mutation::new(ToggleTodo(id)));
        let delete_mutation = use_mutation(Mutation::new(DeleteTodo(id)));

        TableRow::new()
            .child(
                TableCell::new().child(
                    Button::new()
                        .on_press(move |_| toggle_mutation.mutate(id))
                        .child(if self.completed { "✓" } else { "○" }),
                ),
            )
            .child(TableCell::new().child(self.title.clone()))
            .child(
                TableCell::new().child(
                    Button::new()
                        .on_press(move |_| delete_mutation.mutate(id))
                        .child("Delete"),
                ),
            )
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.id)
    }
}

fn app() -> impl IntoElement {
    use_provide_context(|| Captured(init_db()));
    let todos_query = use_query(Query::new((), GetTodos));
    let add_mutation = use_mutation(Mutation::new(AddTodo));
    let mut input_text = use_state(String::new);

    let on_add = move |_| {
        let text = input_text.read().clone();
        if !text.is_empty() {
            add_mutation.mutate(text);
            input_text.set(String::new());
        }
    };

    rect()
        .expanded()
        .padding(16.)
        .spacing(12.)
        .child("SQLite Todo List")
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .cross_align(Alignment::Center)
                .child(Input::new(input_text).placeholder("Add a new todo..."))
                .child(Button::new().on_press(on_add).child("Add")),
        )
        .child(match &*todos_query.read().state() {
            QueryStateData::Loading { res: None } | QueryStateData::Pending => {
                "Loading...".into_element()
            }
            QueryStateData::Loading {
                res: Some(Ok(todos)),
            }
            | QueryStateData::Settled { res: Ok(todos), .. } => {
                if todos.is_empty() {
                    "No todos yet. Add one above!".into_element()
                } else {
                    Table::new()
                        .column_widths([Size::px(60.), Size::flex(1.), Size::px(130.)])
                        .child(
                            TableHead::new().child(
                                TableRow::new()
                                    .child(TableCell::new().child("Status"))
                                    .child(TableCell::new().child("Title"))
                                    .child(TableCell::new().child("Actions")),
                            ),
                        )
                        .child(
                            TableBody::new().child(
                                ScrollView::new()
                                    .children(todos.iter().map(|todo| TodoRow::new(todo).into())),
                            ),
                        )
                        .into_element()
                }
            }
            QueryStateData::Loading {
                res: Some(Err(e)), ..
            }
            | QueryStateData::Settled { res: Err(e), .. } => format!("Error: {}", e).into_element(),
        })
}

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(700., 500.)
                .with_title("SQLite Todo List"),
        ),
    );
}
