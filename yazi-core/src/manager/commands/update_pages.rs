use yazi_shared::{emit, event::Exec, fs::Url, Layer};

use crate::{manager::Manager, tasks::Tasks};

#[derive(Default)]
pub struct Opt {
	page:    Option<usize>,
	only_if: Option<Url>,
}

impl From<&Exec> for Opt {
	fn from(e: &Exec) -> Self {
		Self {
			page:    e.args.first().and_then(|s| s.parse().ok()),
			only_if: e.named.get("only-if").map(Url::from),
		}
	}
}

impl From<()> for Opt {
	fn from(_: ()) -> Self { Self::default() }
}

impl Manager {
	#[inline]
	pub fn _update_pages() {
		emit!(Call(Exec::call("update_pages", vec![]).vec(), Layer::Manager));
	}

	#[inline]
	pub fn _update_pages_by(page: usize, only_if: &Url) {
		emit!(Call(
			Exec::call("update_pages", vec![page.to_string()]).with("only-if", only_if.to_string()).vec(),
			Layer::Manager
		));
	}

	pub fn update_pages(&mut self, opt: impl TryInto<Opt>, tasks: &Tasks) {
		let Ok(opt) = opt.try_into() else {
			return;
		};

		if opt.only_if.is_some_and(|u| u != self.current().cwd) {
			return;
		}

		let targets = self.current().paginate(opt.page.unwrap_or(self.current().page));
		tasks.preload_paged(targets, &self.mimetype);
	}
}
