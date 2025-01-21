fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "proto/api";
    let protos = vec![
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
        "devices/v1/model",
        "devices/v1/devices",
        "auth/v1/model",
        "auth/v1/auth",
        "emails/v1/model",
        "emails/v1/emails",
        "phones/v1/model",
        "phones/v1/phones",
    ];

    let protos: Vec<_> = protos
        .iter()
        .map(|file| format!("{proto_root}/{file}.proto"))
        .collect();

    tonic_build::configure()
    .build_server(true)
    .compile(&protos, &["proto", "googleapis"])?;

     //::compile_protos("proto/api/user/v1/user.proto")?; //api/users/v1/
    Ok(())
}