use crate::frameworks::nextjs::auth_file::write_project_file;

pub fn create_client_file(location_choice: &str, app_url_env_var: &str) -> std::io::Result<()> {
    let content = format!(
        "import {{\n\
         \tcreateAuthClient\n\
         }} from \"better-auth/react\";\n\n\
         export const authClient = createAuthClient({{\n\
         \tbaseURL: {},\n\
         }})\n\n\
         export const {{\n\
         \tsignIn,\n\
         \tsignOut,\n\
         \tsignUp,\n\
         \tuseSession\n\
         }} = authClient;",
        app_url_env_var
    );
    write_project_file(location_choice, "auth-client.ts", &content)
}
