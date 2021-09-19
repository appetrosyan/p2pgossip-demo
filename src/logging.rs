pub trait LogExt {
	fn log(self) -> Self;
}

impl<T, E> LogExt for Result<T, E>
where
	E: std::fmt::Display,
{
	fn log(self) -> Self {
		if let Err(e) = &self {
			log::error!("{}", e)
		}
		self
	}
}
