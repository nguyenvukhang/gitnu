use std::path::{Component, Path, PathBuf};

pub fn diff_paths<P, B>(path: P, base: B) -> Option<PathBuf>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let (path, base) = (path.as_ref(), base.as_ref());
    if path.is_absolute() != base.is_absolute() {
        path.is_absolute().then(|| PathBuf::from(path))
    } else {
        let (mut ita, mut itb) = (path.components(), base.components());
        let mut cs = vec![];
        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    cs.push(a);
                    cs.extend(ita.by_ref());
                    break;
                }
                (None, _) => cs.push(Component::ParentDir),
                (Some(a), Some(b)) if cs.is_empty() && a == b => (),
                (Some(a), Some(b)) if b == Component::CurDir => cs.push(a),
                (Some(_), Some(b)) if b == Component::ParentDir => return None,
                (Some(a), Some(_)) => {
                    cs.push(Component::ParentDir);
                    for _ in itb {
                        cs.push(Component::ParentDir);
                    }
                    cs.push(a);
                    cs.extend(ita.by_ref());
                    break;
                }
            }
        }
        Some(cs.iter().map(|c| c.as_os_str()).collect())
    }
}
