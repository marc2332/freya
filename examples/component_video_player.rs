#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

const MARKDOWN_CONTENT: &str = r#"# freya-video

Componente de reproductor de video para [Freya](https://freyaui.dev/), basado en **FFmpeg** para la decodificación y **Rodio** para la reproducción de audio.

## Feature flags

| Feature | Descripcion |
|---|---|
| `video` | Habilita el reproductor en `freya` / `freya-components` |
| `streaming` | Habilita `VideoSource::url()` para streams de red (HTTP, HLS, RTMP...) |

```toml
# Solo reproduccion de archivos locales
[dependencies]
freya = { version = "0.4.0-rc.16", features = ["video"] }

# Con soporte de streaming
[dependencies]
freya-video = { version = "0.4.0-rc.16", features = ["streaming"] }
```

## Requisitos del sistema

FFmpeg debe estar instalado en el sistema (se enlaza en tiempo de compilación via `ffmpeg-next`).

## Uso

### Básico

```rust
use freya::prelude::*;
use std::path::PathBuf;

fn app() -> impl IntoElement {
    VideoPlayer::new(PathBuf::from("./video.mp4"))
        .width(Size::px(640.))
        .height(Size::px(360.))
}
```

### Props

| Método | Tipo | Por defecto | Descripción |
|---|---|---|---|
| `new(source)` | `impl Into<VideoSource>` | — | Ruta al archivo de video |
| `.width()` / `.height()` | `Size` | — | Tamaño en el layout |
| `.autoplay(bool)` | `bool` | `true` | Reproducir automáticamente al renderizar |
| `.hide_controls(bool)` | `bool` | `false` | Ocultar el overlay de controles integrado |
| `.custom_controls(Rc<dyn Fn(VideoControls) -> Element>)` | — | `None` | Reemplazar el overlay integrado por uno propio |

### `VideoSource`

```rust
// Desde un string de ruta
let source: VideoSource = "./video.mp4".into();

// Desde un PathBuf
let source: VideoSource = PathBuf::from("./video.mp4").into();
```

### Streaming (feature `streaming`)

Con el feature `streaming` habilitado, `VideoSource::url()` acepta cualquier URL que FFmpeg soporte: HTTP/HTTPS directo, HLS (`.m3u8`), RTMP, RTSP, entre otros.

```rust
use freya::prelude::*;

fn app() -> impl IntoElement {
    VideoPlayer::new(VideoSource::url("https://example.com/stream.m3u8"))
        .width(Size::fill())
        .height(Size::fill())
}
```

> FFmpeg debe haber sido compilado con soporte para el protocolo correspondiente.
> En instalaciones via `brew`, `apt` o los binarios oficiales esto ya viene incluido por defecto.

### Controles personalizados

Reemplaza el overlay integrado con tu propia UI:

```rust
use std::rc::Rc;
use freya::prelude::*;

fn app() -> impl IntoElement {
    VideoPlayer::new("./video.mp4")
        .custom_controls(Rc::new(|c: VideoControls| {
            rect()
                .horizontal()
                .child(if c.is_playing { "Reproduciendo" } else { "Pausado" })
                .child(format!("{:.0}s / {:.0}s", c.current_secs, c.total_secs))
                .into_element()
        }))
}
```

#### Campos de `VideoControls`

| Campo | Tipo | Descripción |
|---|---|---|
| `is_playing` | `bool` | El video se está reproduciendo activamente |
| `is_paused` | `bool` | El video está pausado |
| `is_finished` | `bool` | La reproducción ha terminado |
| `progress` | `f64` | Posición como fracción `0.0..=1.0` |
| `current_secs` | `f64` | Posición actual en segundos |
| `total_secs` | `f64` | Duración total en segundos |
| `volume` | `f64` | Volumen actual `0.0..=1.0` |
| `toggle_play` | `Rc<dyn Fn()>` | Alternar play/pausa (reinicia desde el inicio si terminó) |
| `seek` | `Rc<dyn Fn(f64)>` | Saltar a una fracción de la duración total |
| `set_volume` | `Rc<dyn Fn(f64)>` | Establecer el volumen |
"#;

fn app() -> impl IntoElement {
    ScrollView::new()
        .width(Size::fill())
        .height(Size::flex(1.))
        .child(
            rect()
                .vertical()
                .spacing(18.)
                .child(
                    VideoPlayer::new(PathBuf::from("./examples/sample.mp4"))
                        .width(Size::fill())
                        .height(Size::px(450.))
                        .autoplay(false),
                )
                .child(MarkdownViewer::new(MARKDOWN_CONTENT).padding(18.)),
        )
}
