
sig Card {}

sig World {
	cards_drawn: set Card,
	card1: Card,
	card2: Card,
	hidden: Card,
	access: set World
}{
	#cards_drawn = 3
	cards_drawn = card1 + card2 + hidden
}

// Worlds: one for each subset
fact worlds_distinct {
	all disj w,w': World | w.cards_drawn != w'.cards_drawn
}
fact worlds_exist {
	all disj c1, c2, c3: Card | some w: World | w.cards_drawn = c1 + c2 + c3
	/*all draw: set Card | (#draw = 3) => (some w: World | w.cards_drawn = draw)*/ // bad -- higher-order quantification problems
}

// What is known: card1 and card2 only
fact cards_seen {
	all w: World | w.access = {w': World | w.card1 = w'.card1 and w.card2 = w'.card2}
}

// SOLVING THE PUZZLE
pred knows_hidden[w: World] {
	all w': w.access | w.hidden = w'.hidden
}
run knows_hidden for exactly 8 Card, exactly 56 World // works...but takes SEVERAL minutes to run.
