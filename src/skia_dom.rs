use dioxus::core::Mutations;

pub trait SkiaElement {
    fn render(&self);
}

pub struct Div;
impl SkiaElement for Div {
    fn render(&self){

    }
}

pub struct SkiaDom {
    pub root: Option<Box<dyn SkiaElement>>
}
impl SkiaDom {

    pub fn from_mutation(mutations: Vec<Mutations>) -> Self {

        let root = None;

        for mutt in mutations {
            for e in mutations.edits {
                match e {
                    PushRoot { root } => self.node_stack.push(root as usize),
                    AppendChildren { many } => {

                    }
                    _ => {

                    }
                };
            }
        }
        //Some(Box::new(root))

        Self {
            root,
        }
    }
}