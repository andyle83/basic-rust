pub struct Leave {
    pub color: String,
}

pub fn print_leave(leave: &Leave) {
    println!("Leave color: {:?}", leave.color);
}