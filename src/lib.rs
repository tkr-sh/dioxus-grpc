use {
    convert_case::{Case, Casing},
    std::{fmt::Write, path::Path},
    tonic_build::Config,
};

/// - to_path: Is the directory in which files should be written to. When [`None`], defaults to `OUT_DIR`
pub fn generate_hooks<P: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>>(
    protos: &[P],
    includes: &[P2],
    to_path: &Option<P3>,
    uri: &str,
) -> Result<(), std::io::Error> {
    let mut config = Config::new();
    let file_descriptor_set = config.load_fds(protos, includes)?;

    for fd in file_descriptor_set.file {
        let mut str = String::new();
        let pkg_name = fd
            .package
            .as_ref()
            .map_or_else(|| "_", |string| string.as_str());
        let filename = format!("{pkg_name}.dx.rs");

        for service in &fd.service {
            write!(
                str,
                r#"
                #[path = "{out_dir}/{package_name}.rs"]
                mod proto;
                pub use proto::*;
                use ::dioxus::prelude::*;

                pub struct {service_name}ServiceHook({tonic_client}<::tonic_web_wasm_client::Client>);

                pub fn use_{service_name_lowercase}_service() -> {service_name}ServiceHook {{
                    {service_name}ServiceHook({tonic_client}::new(::tonic_web_wasm_client::Client::new(
                        {uri:?}.to_string()
                    )))
                }}

                impl {service_name}ServiceHook {{
                "#,
                out_dir = std::env::var("OUT_DIR").expect("build.rs"),
                tonic_client = format!(
                    "proto::{}_client::{}Client",
                    service.name().to_case(Case::Snake),
                    service.name().to_case(Case::Pascal)
                ),
                package_name = pkg_name,
                service_name = service.name().to_case(Case::Pascal),
                service_name_lowercase = service.name().to_case(Case::Snake)
            ).expect("write error");

            for rpc in &service.method {
                write!(
                    str,
                    r"
                    pub fn {rpc_name}(&self, req: Signal<proto::{rpc_input}>) -> Resource<Result<proto::{rpc_ouptut}, tonic::Status>> {{
                        let client = self.0.to_owned();
                        use_resource(move || {{
                            let mut client = client.clone();
                            async move {{ client.{rpc_name}(req()).await.map(|resp| resp.into_inner()) }}
                        }})
                    }}
                    ",
                    rpc_name = rpc.name().to_case(Case::Snake),
                    rpc_input = rpc
                        .input_type()
                        .split('.')
                        .next_back()
                        .expect("Should always have type"),
                    rpc_ouptut = rpc
                        .output_type()
                        .split('.')
                        .next_back()
                        .expect("Should always have type"),
                ).expect("write error");
            }

            str.push('}');
        }

        match to_path {
            Some(p) => {
                std::fs::write(
                    {
                        let mut path_to_file = p.as_ref().to_owned();
                        path_to_file.push(filename);
                        path_to_file
                    },
                    str,
                )
            },
            None => {
                std::fs::write(
                    format!("{}/{filename}", std::env::var("OUT_DIR").expect("build.rs")),
                    str,
                )
            },
        }?;
    }


    Ok(())
}
