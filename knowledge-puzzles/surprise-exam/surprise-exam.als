
/****************
***   Setup   ***
****************/

open util/ordering[Time] as time

// Don't need people in this one -- there's just one person, the student

// Data: the time (day) that the test occurs.
// Days are: Sunday, Monday, Tuesday, Wednesday, Thursday, Friday. But the test does not occur
// on Sunday, which is the initial day.
sig World {
	exam_day: Time
}{
	exam_day != time/first
}
one sig RealWorld extends World {}

// Time: Monday, Tuesday, Wednesday, Thursday, or Friday
sig Time {
	/*domain : set World, // common knowledge*/
	access : World -> World, // knowledge of student at this point
}

/****************************
***   Fundamental facts   ***
****************************/

// S5 axioms -- equivalence relation
fact s5 {
	all t : Time | {
		// reflexive
		all w : World | w in t.access[w]
		// symmetric and transitive (if u->v and u->w then v->w)
		all w : World | all w' : t.access[w] | all w'' : t.access[w] | w'' in t.access[w']
	}
}

// Different worlds are different -- weaker version
fact different_worlds_are_different {
	all disj w, w': World {
		w.exam_day != w'.exam_day or
		some t: Time | t.access[w] != t.access[w']
	}
}

/*******************************************
***   What the student knows over time   ***
*******************************************/

pred transition[t,t': Time] {
	// You now know everything you did before, and also that the exam either was on this day or wasn't.
	let yes_exam = exam_day.t' |
	let no_exam = univ - yes_exam |
		t'.access = t.access & ((yes_exam -> yes_exam) + (no_exam -> no_exam))
}

fact transitions {
	all t : Time - time/last | let t' = time/next[t] | transition[t,t']
}

/**********************************
***   The puzzle and solution   ***
**********************************/

pred knows_exam_day[w: World, t: Time] {
	all w': t.access[w] | w'.exam_day = w.exam_day
}

pred is_unexpected[w: World] {
	// The exam was unexpected
	// At the time the day before the test, the student did not know the exam would occur on that day
	let day_before = time/prev[w.exam_day] | not knows_exam_day[w,day_before]
}

pred unexpected_exam {
	is_unexpected[RealWorld]
}
run unexpected_exam for 10 World, exactly 2 Time // no instance -- it has to be expected
run unexpected_exam for 10 World, exactly 3 Time // instance found
run unexpected_exam for 10 World, exactly 4 Time // instance found
run unexpected_exam for 10 World, exactly 6 Time // instance found

pred knows_unexpected_exam{
	all w': time/first.access[RealWorld] | is_unexpected[w']
}
run knows_unexpected_exam for 10 World, exactly 2 Time
run knows_unexpected_exam for 10 World, exactly 3 Time
run knows_unexpected_exam for 10 World, exactly 4 Time
run knows_unexpected_exam for 10 World, exactly 6 Time
// None of the above have instances. It's impossible to know that the exam will be unexpected!

//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
/*
2016-05-02 ADDENDUM
New definition of "surprise exam"
An exam on day x is a surprise if there is a day y
such that you did not know the exam would be on day x on day y.
(Loose interpretation of Sammy's definition of surprise)
(Now you are only allowed to expect the exam on one day. The exam is a surprise if when
you get to at some point choose to expect the exam on some day, and then you aren't right.)
*/

pred is_unexpected_2[w: World] {
	// The exam was unexpected ON SOME DAY
	some t: Time | not knows_exam_day[w,t]
}

// This isn't really important to illustrate
//pred unexpected_exam_2 {
//	is_unexpected_2[RealWorld]
//}
//run unexpected_exam_2 for 10 World, exactly 2 Time // no instance -- it has to be expected
//run unexpected_exam_2 for 10 World, exactly 3 Time // instance found
//run unexpected_exam_2 for 10 World, exactly 4 Time // instance found
//run unexpected_exam_2 for 10 World, exactly 6 Time // instance found

pred knows_unexpected_exam_2{
	all w': time/first.access[RealWorld] | is_unexpected_2[w']
}
run knows_unexpected_exam_2 for 10 World, exactly 2 Time // still no instances
run knows_unexpected_exam_2 for 10 World, exactly 3 Time // has instances
run knows_unexpected_exam_2 for 10 World, exactly 4 Time // has instances
run knows_unexpected_exam_2 for 10 World, exactly 6 Time // has instances
// Notice that now, the exam can be "unexpected" and the student can even know it.
