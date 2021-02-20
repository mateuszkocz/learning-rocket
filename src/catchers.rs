use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
  format!("The {} path doesn't exist.", req.uri())
}