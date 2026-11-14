use std::io;

use freya_core::data_transfer::{
    DataFormat,
    DataValue,
    TransferData,
};
use url::Url;
use winit::data_transfer::{
    DataTransferSend,
    DataTransferSendBuilder,
    SendData,
    TypeHint,
    TypedData,
};

pub(crate) trait DataFormatExt {
    fn to_type_hint(self) -> TypeHint;
    fn read(self, value: &dyn TypedData) -> io::Result<DataValue>;
}

impl DataFormatExt for DataFormat {
    fn to_type_hint(self) -> TypeHint {
        match self {
            Self::Files => TypeHint::UriList,
            Self::Text => TypeHint::Plaintext,
            Self::Html => TypeHint::Html,
            Self::Rtf => TypeHint::Rtf,
            Self::Image => TypeHint::Image {
                extension_hint: Some("png"),
            },
            Self::Audio => TypeHint::Audio {
                extension_hint: Some("wav"),
            },
        }
    }

    fn read(self, value: &dyn TypedData) -> io::Result<DataValue> {
        match self {
            Self::Files => {
                let paths = value
                    .try_as_uris()?
                    .into_iter()
                    .map(|uri| {
                        Url::parse(&uri)
                            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?
                            .to_file_path()
                            .map_err(|_| {
                                io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "A transferred URI is not a local file",
                                )
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(DataValue::Files(paths))
            }
            Self::Text | Self::Html | Self::Rtf => value.try_as_string().map(DataValue::Text),
            Self::Image | Self::Audio => value.try_as_bytes().map(DataValue::Bytes),
        }
    }
}

pub(crate) trait TransferDataExt {
    fn into_winit(self) -> io::Result<Box<dyn DataTransferSend>>;
}

impl TransferDataExt for TransferData {
    fn into_winit(self) -> io::Result<Box<dyn DataTransferSend>> {
        if self.formats.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "A transfer needs at least one data format",
            ));
        }

        let mut builder = DataTransferSendBuilder::new(());
        for (format, value) in self.formats {
            let value = match (format, value) {
                (DataFormat::Files, DataValue::Files(paths)) => {
                    let uris = paths
                        .iter()
                        .map(|path| {
                            Url::from_file_path(path)
                                .map(|url| url.to_string())
                                .map_err(|_| {
                                    io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "Transferred file paths must be absolute",
                                    )
                                })
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    SendData::Uris(uris)
                }
                (DataFormat::Files, _) | (_, DataValue::Files(_)) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "File paths require the Files format and a file path payload",
                    ));
                }
                (_, DataValue::Text(text)) => SendData::String(text),
                (_, DataValue::Bytes(bytes)) => SendData::Bytes(bytes),
            };

            builder.add_type(format.to_type_hint(), move |_, _| Some(value.clone()));
        }

        Ok(builder.build())
    }
}
