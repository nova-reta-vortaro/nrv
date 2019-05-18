fn main() -> ructe::Result<()> {
	ructe::Ructe::from_env()?.compile_templates("templates")
}
