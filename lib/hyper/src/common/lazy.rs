use pin_project::pin_project;

use super::{task, Future, Pin, Poll};

pub(crate) trait Started: Future {
  fn started(&self) -> bool;
}

pub(crate) fn lazy<F, R>(func: F) -> Lazy<F, R>
where
  F: FnOnce() -> R,
  R: Future + Unpin, {
  Lazy { inner: Inner::Init(func) }
}

// FIXME: allow() required due to `impl Trait` leaking types to this lint
#[allow(missing_debug_implementations)]
#[pin_project]
pub(crate) struct Lazy<F, R> {
  #[pin]
  inner: Inner<F, R>,
}

#[pin_project(project = InnerProj, project_replace = InnerProjReplace)]
enum Inner<F, R> {
  Init(F),
  Fut(#[pin] R),
  Empty,
}

impl<F, R> Started for Lazy<F, R>
where
  F: FnOnce() -> R,
  R: Future,
{
  fn started(&self) -> bool {
    match self.inner {
      Inner::Init(_) => false,
      Inner::Fut(_) | Inner::Empty => true,
    }
  }
}

impl<F, R> Future for Lazy<F, R>
where
  F: FnOnce() -> R,
  R: Future,
{
  type Output = R::Output;

  fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
    let mut this = self.project();

    if let InnerProj::Fut(f) = this.inner.as_mut().project() {
      return f.poll(cx);
    }

    match this.inner.as_mut().project_replace(Inner::Empty) {
      InnerProjReplace::Init(func) => {
        this.inner.set(Inner::Fut(func()));
        if let InnerProj::Fut(f) = this.inner.project() {
          return f.poll(cx);
        }
        unreachable!()
      }
      _ => unreachable!("lazy state wrong"),
    }
  }
}
