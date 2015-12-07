use dtl::Value;

const HUNDREDS: [&'static str; 10] = [
	"",
	"сто",
	"двести",
	"триста",
	"четыреста",
	"пятьсот",
	"шестьсот",
	"семьсот",
	"восемьсот",
	"девятьсот",
];

const TENS: [&'static str; 10] = [
	"",
	"",
	"двадцать",
	"тридцать",
	"сорок",
	"пятьдесят",
	"шестьдесят",
	"семьдесят",
	"восемьдесят",
	"девяносто",
];

const ONES: [&'static str; 20] = [
	"",
	"одна",
	"две",
	"три",
	"четыре",
	"пять",
	"шесть",
	"семь",
	"восемь",
	"девять",
	"десять",
	"одиннадцать",
	"двенадцать",
	"тринадцать",
	"четырнадцать",
	"пятнадцать",
	"шестнадцать",
	"семнадцать",
	"восемнадцать",
	"девятнадцать",
];

fn group_to_handwritten(should_add_thousand: bool, mut group: u32) -> String {
	let mut res = Vec::<&str>::new();
	if group >= 100 {
		res.push(HUNDREDS[(group / 100) as usize]);
		group %= 100;
	}
	if group >= 20 {
		res.push(TENS[(group / 10) as usize]);
		group %= 10;
	}
	if group > 0 {
		res.push(ONES[group as usize]);
	}
	if should_add_thousand {
		res.push(match group {
			1 => "тысяча",
			2...5 => "тысячи",
			_ => "тысяч",
		});
	}
	res.join(" ")
}

fn u32_to_handwritten(mut num: u32) -> String {
	let mut groups = Vec::new();
	while num > 0 {
		groups.push(num % 1000);
		num /= 1000;
	}
	let groups_as_strings: Vec<String> = groups.iter().rev()
		.enumerate()
		.map(|(idx, group)| group_to_handwritten(idx < groups.len() - 1, *group))
		.collect();
	groups_as_strings.join(" ")
}

pub fn handwritten_sum(input: Option<Box<Value>>, _: &str) -> Option<Box<Value>> {
	match input {
		None => None,
		Some(content) => match content.downcast_ref::<f32>() {
			None => None,
			Some(num) => {
				let res = u32_to_handwritten(*num as u32);
				Some(Box::new(res))
			}
		}
	}
}