fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "proto/api";
    let protos = vec![
        // "auth",
        // "device",
        // "hooks",
        // "idp",
        // "oidc",
        // "org",
        // "premonition",
        // "resource",
        // "roles",
        // "saml",
        // "sessions",
        "users/v1/model",
        "users/v1/user",
    ];

    let protos: Vec<_> = protos
        .iter()
        .map(|file| format!("{proto_root}/{file}.proto"))
        .collect();

    tonic_build::configure()
    .build_server(true)
    .compile(&protos, &["proto"])?;

     //::compile_protos("proto/api/user/v1/user.proto")?; //api/users/v1/
    Ok(())
}