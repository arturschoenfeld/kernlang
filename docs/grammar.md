program
	statement+

statement
	var_stmt /
	fn_stmt /
	use_stmt /
	expr_stmt

expr_stmt
	term (binary_op term)* /
	(unary_op term)* /
	if_expr /
	else_expr /
	match_expr /

term
	integer /
	float

integer
	["+"|"-"] digit+

digit
	0..=9