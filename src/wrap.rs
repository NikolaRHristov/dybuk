use std::iter::repeat;

pub fn wrap_msg(s:String, n:usize) -> String {
	let split = s.split(' ').collect::<Vec<_>>();
	let mut chunks = split.chunks(8).map(|x| x.join(" "));

	chunks.next().unwrap_or("?".to_string())
		+ &chunks
			.map(|ref x| {
				"\n           >  ".to_string() + &repeat(" ").take(n).collect::<String>() + x
			})
			.collect::<String>()
}
