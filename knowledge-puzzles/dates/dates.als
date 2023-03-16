
/****************
***   Setup   ***
****************/

sig World {
	month: Int,
	day: Int
}
one sig RealWorld extends World {}

abstract sig Person {
	access: World -> World
}
one sig A extends Person {}
one sig B extends Person {}

/****************************
***   Fundamental facts   ***
****************************/

// S5 axioms -- equivalence relation
fact S5 {
	all p : Person | {
		// reflexive
		all w : World | w -> w in p.access
		// symmetric and transitive (if u->v and u->w then v->w)
		all w : World | all w' : p.access[w] | all w'' : p.access[w] | w'' in p.access[w']
	}
}

/*
	Simplifying assumption -- different worlds have different data.
	- In other words, we are imagining that what each person knows (before the conversation starts) is only a function of the state of the world. It's deterministic: if the world is a certain way, then you know a certain set of things, and nothing more. There's not a possibility that you know more or less despite the situation being identical.
	- Is this necessary?
*/
fact DifferentWorldsAreDifferent {
	all w : World | all w' : World {
		(w.month = w'.month and w.day = w'.day) => w = w'
	}
}

/***************************
***   Common knowledge   ***
***************************/

// Cheryl's birthday is one of 3/4, 3/5, 3/8, 6/4, 6/7, 9/1, 9/5, 12/1, 12/2, 12/8
// We have 16 integers -- so these dates don't overflow.
fact {
	all w: World | let m = w.month | let d = w.day | {
		   (m = 3  and d = 4) //
		or (m = 3  and d = 5) //
		or (m = 3  and d = 8)
		or (m = 6  and d = 4) //
		or (m = 6  and d = 7) //
		or (m = 9  and d = 1)
		or (m = 9  and d = 5)
		or (m = 12 and d = 1)
		or (m = 12 and d = 2)
		or (m = 12 and d = 8)
	}
}
// A knows the month -- AND NOTHING MORE!
fact {
	all w: World |
		A.access[w] = {w': World | w.month = w'.month}
}
// B knows the day -- AND NOTHING MORE!
fact {
	all w: World |
		B.access[w] = {w': World | w.day = w'.day}
}

run basic_setup {} for 10 World, 2 Person, 5 int

/*******************
***   Exchange   ***
*******************/
// for brevity -- knows birthday
pred knows_bd[p: Person, w: World, u: set World] {
	all w1: (p.access[w] & u) |
	all w2: (p.access[w] & u) |
		w1.month = w2.month and w1.day = w2.day
}

// 1. A doesn't know, A knows B doesn't know either
pred statement1[w: World] {
	not knows_bd[A,w,World]
	all w': A.access[w] | not knows_bd[B,w',World]
}
// 2. Within universe1, B knows.
pred statement2[w: World] {
	statement1[w]
	let u1 = {x : World | statement1[x]} | knows_bd[B,w,u1]
}
// 3. Within universe2, A knows.
pred statement3[w: World] {
	statement2[w]
	let u2 = {x : World | statement1[x] and statement2[x]} | knows_bd[A,w,u2]
}

// Conclusion
fact conversation {
	statement3[RealWorld]
}

assert its_september_1 {
	RealWorld.month = 9
	RealWorld.day = 1
}
check its_september_1 for 10 World, 2 Person, 5 int // still get counterexamples -- because maybe there isn't a world for every date.

check its_september_1 for exactly 10 World, 2 Person, 5 int
// no counterexample!
// Takes a few minutes to run.

run find_instance {} for exactly 10 World, 2 Person, 5 int // View the solution.
// Runs much more quickly.
