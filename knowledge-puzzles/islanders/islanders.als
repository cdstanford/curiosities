
/****************
***   Setup   ***
****************/

open util/ordering[Time] as time

// Want to handle an arbitrary number of people this time.
sig Person {}

// Data: eye color
abstract sig Color {}
one sig Blue extends Color {}
one sig Brown extends Color {}

sig World {
	access: Person -> World, // join with a specific person to get a specific access relation
	eyes: Person -> one Color
}
one sig RealWorld extends World {}

/*
	For this puzzle, we need a sequence of times.
	Each time has a set of worlds, and represents the current state of "common knowledge".
	The set of worlds is decreasing over time (i.e., the future set of worlds is a subset of the prior set of worlds.
	People don't forget things.)
	Note, that the accessability relation, defined above, will not change.
*/
// set of times 'Time' and (defined earlier) an ordering on them, 'time'.
// Also in the time include who has revealed that they know their own eye color--see puzzle description.
sig Time {
	domain : set World,
	others_domain: set World, -- Everything in domain except real world. This is just for displaying purposes...
	revealed_eyes : set Person
}{
	others_domain = domain - RealWorld -- as stated above, just for displaying
}

/****************************
***   Fundamental facts   ***
****************************/

// S5 axioms -- equivalence relation
fact s5 {
	all p : Person | {
		// reflexive
		all w : World | w in w.access[p]
		// symmetric and transitive (if u->v and u->w then v->w)
		all w : World | all w' : w.access[p] | all w'' : w.access[p] | w'' in w'.access[p]
	}
}

// Simplifying assumption -- different worlds have different data.
// As we saw in the date puzzle, this allows us to guarantee one world for each data set simply by
// specifying the number of worlds. So it's convenient in that sense.

fact different_worlds_are_different {
	all w : World | all w' : World {
		// If all people have the same color of eyes in w and in w', then w = w'
		(all p: Person | w.eyes[p] = w'.eyes[p]) => w = w'
	}
}

/***************************
***   What people know   ***
***************************/

// Everyone sees everyone else's eye color, but not their own.
fact who_sees_what {
	all w: World | all p: Person | {
		w.access[p] = {w': World | (all p': Person - p | w.eyes[p'] = w'.eyes[p'])}
	}
}

/***********************
***   Initial time   ***
***********************/

fact initial {
	let t0 = time/first | {
		// Initial common knowledge: "There is at least one person with Blue eyes."
		// Can edit this line for other interesting situations.
		t0.domain = {w : World | (some p: Person | w.eyes[p] = Blue)}
		// The statement that at least one person has blue eyes had better be true.
		RealWorld in t0.domain
		// Finally, initially, no one has revealed their eye color.
		no t0.revealed_eyes
	}
}

run basic_instance {} for exactly 3 Person, exactly 8 World, exactly 1 Time

/*********************
***   Trasitions   ***
*********************/

pred knowsEyes[p: Person, w: World, d: set World] {
	// d is the domain (current time)
	w in d // only really makes sense if w in d
	all w': d & w.access[p] | {
		w'.eyes[p] = w.eyes[p]
	}
}

pred transition[t,t': Time] {
		// First, those who know in time t their eye color reveal their eyes.
		t'.revealed_eyes = {p: Person | knowsEyes[p, RealWorld, t.domain] }
		// Second, we use this to update the domain:
		// - Those who revealed their eye color actually know their eye color.
		// - Those who didn't reveal didn't know their eye color.
		t'.domain = {w: t.domain | {
			all p: t'.revealed_eyes | knowsEyes[p, w, t.domain]
			all p: Person - t'.revealed_eyes | not knowsEyes[p, w, t.domain]
		}}
}

fact transitions {
	all t : Time - time/last | let t' = time/next[t] | transition[t,t']
}

run timed_instance {} for exactly 1 Person, exactly 2 World, exactly 4 Time
run timed_instance {} for exactly 2 Person, exactly 4 World, exactly 5 Time
run timed_instance {} for exactly 3 Person, exactly 8 World, exactly 6 Time
run timed_instance {} for exactly 4 Person, exactly 16 World, exactly 7 Time -- didn't wait for this to finish.

assert someContainments {
	all t: Time - time/last | let t' = time/next[t] {
		t.revealed_eyes in t'.revealed_eyes
		t'.domain in t.domain
	}
}
check someContainments for exactly 1 Person, exactly 2 World, exactly 4 Time
check someContainments for exactly 2 Person, exactly 4 World, exactly 5 Time
check someContainments for exactly 3 Person, exactly 8 World, exactly 6 Time
check someContainments for exactly 4 Person, exactly 16 World, exactly 7 Time

/**************************************
***   The solution to the puzzle!   ***
**************************************/
fun timeBeforeReveal[p: Person] : Int {
	#{t: Time | p not in t.revealed_eyes}
}
assert solution {
	let BlueP = {p: Person | RealWorld.eyes[p] = Blue} |
	let BrownP = {p: Person | RealWorld.eyes[p] = Brown} |
	all p: Person | {
		// m blue, n brown
		// All blue-eyes reveal after m days
		// All brown-eyes reveal after m+1 days.
		p in BlueP => timeBeforeReveal[p] = #BlueP
		p in BrownP => timeBeforeReveal[p] = #BlueP + 1
	}
}
check solution for exactly 1 Person, exactly 2 World, exactly 4 Time
check solution for exactly 2 Person, exactly 4 World, exactly 5 Time
check solution for exactly 3 Person, exactly 8 World, exactly 6 Time
check solution for exactly 4 Person, exactly 16 World, exactly 7 Time
// All of the above are successful! :)
