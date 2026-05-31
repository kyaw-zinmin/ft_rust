mod utils;
use ft_rust::ft;

#[test]
fn test_putstr() {
	let cases: &[(&str, &str, &str)] = &[
		(	"A single string",
			"hello world",
			"hello world"
		),
		(	"An empty string",
			"",
			""
		),
		(	"A string containing numbers",
			"0123456789",
			"0123456789"
		),
		(	"A long string",
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse lectus tortor, dignissim sit amet, adipiscing nec, ultricies sed, dolor. Cras elementum ultrices diam. Maecenas ligula massa, varius a, semper congue, euismod non, mi.",
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse lectus tortor, dignissim sit amet, adipiscing nec, ultricies sed, dolor. Cras elementum ultrices diam. Maecenas ligula massa, varius a, semper congue, euismod non, mi."
		)
	];
	utils::compare_cases(cases, ft::putstr);
}