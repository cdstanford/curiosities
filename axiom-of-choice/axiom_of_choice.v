(*
Problem: write a statement of the Axiom of Choice in Coq that
(1) typechecks
(2) is provable

Thanks to Li-yao Xia, Solomon Maina, Yannick Zakowski, and Yao Li for
discussions at Penn and over email, November 2018.

Link where I got choice1 and choice2 from:
https://coq-club.inria.narkive.com/zfXJIT7X/axiom-of-choice
*)

Check existT.

Theorem choice1:
forall A B : Set,
forall phi : A -> B -> Set,
(forall x : A, { y : B & phi x y }) ->
{f : A -> B & forall x : A, phi x (f x)}.
Proof.
  intros A B phi H.
  apply existT with (x :=  fun x => (projT1 (H x))).
  intros x. destruct (H x) as [y Pxy].
  simpl. apply Pxy.
Qed.

Check @choice1.
Check (forall (A B : Set) (phi : A -> B -> Set),
       (forall x : A, {y : B & phi x y}) ->
       {f : A -> B & forall x : A, phi x (f x)}).


Check @ex_intro.

(*Definition get_existential (A : Type) (P : A -> Prop) (H : exists (a : A), P a) : A :=
    match H with
    | @ex_intro _ _ x _ => x
    end.*)
(*
Fails with this error:
  Incorrect elimination of "H" in the inductive type "ex":
  the return type has sort "Type" while it should be "Prop".
  Elimination of an inductive object of sort Prop
  is not allowed on a predicate in sort Type
  because proofs can be eliminated only to build proofs.
*)

(* But this can type-check at least: *)

Theorem get_existential (A : Type) (P : A -> Prop) (H : exists (a : A), P a) : A.
Proof. Abort.

Theorem choice2:
forall A B : Set,
forall phi : A -> B -> Prop,
forall x : A, ex (phi x) ->
ex (fun f : A -> B => forall x : A, phi x (f x)).
Proof. Abort.

(* what if we just replace Set with Prop directly, in choice1? *)
(* Does it typecheck (Li-yao wonders)? *)

Theorem choice3:
forall A B : Set,
forall phi : A -> B -> Prop,
(forall x : A, { y : B & phi x y }) ->
{f : A -> B & forall x : A, phi x (f x)}.
Proof.
  intros A B phi H.
  apply existT with (x :=  fun x => (projT1 (H x))).
  intros x. destruct (H x) as [y Pxy].
  simpl. apply Pxy.
Qed.

(* Weird... *)
