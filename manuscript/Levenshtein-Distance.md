# Levenshtein Distance
The [*Levenshtein distance*][levenshtein], also called the *edit distance*,
measures how different two strings are. It is defined as the minimum number of
insertions, deletions or single character edits necessary to transform one
string into the other.

Below are a few examples where we calculate the Levenshtein distance of a pair
of string.

1. The Levenshtein distance between `banana` and `bananas` is 1 because we can
   insert `s` at the end of `banana` to get `bananas`.
2. For the pair `spoons` and `spoon` the Levenshtein distance is also 1 because
   we can delete the final `s` in `spoons` and get `spoon`.
3. this time the Levenshtein distance is 2 for the strings `saloon` and
   `lagoon`. We need to change the `s` in a `l` and the `l` into a `g` in order
   to change `saloon` into `lagoon`.

## Patterns
When you think about calculating Levenshtein distances it is nice to notice some
patterns. One pattern that could spring to mind is the Levenshtein distance
between a string `s` and the _empty string_, i.e a string with length zero.

If you realize that need to add at least as many characters as `s` has and there
is a sequence of insertions that transforms the empty string into `s`, i.e.
insert each character of `s` one after the other, you figured out that the
Levenshtein distance between `s` and the empty string is exactly the length of
`s`.

An other pattern that we could examine is how does the Levenshtein change if we
slightly alter the strings. Let's imagine two non-empty strings, otherwise we
already know what the Levenshtein distance is. We call them `u` and `v`. There
are three related Levenshtein distances we could examine. The Levenshtein
distance between `u` and `v` with the last character deleted, the Levenshtein
distance between `u` with the last character deleted and `v`, and the
Levenshtein distance between `u` and `v` where we both deleted the last
character.

Let's write `L(u, v)` for the Levenshtein distance between `u` and `v`.
Furthermore let's indicate the string `u` with the last character deleted `u'`.
Now if we know the Levenshtein distances `L(u, v')`, `L(u', v)` and `L(u', v')`
the Levenshtein distance between `L(u, v)` is the most economical choice of one
of the following choices

1. `L(u, v') + 1` by inserting the last character to `v'`.
2. `L(u', v) + 1` by inserting the last character to
   `u'`.
3. `L(u', v') + 1` if the last characters of `u'` and `v'` are different,
   because we need to edit one character into the other.
4. `L(u', v')` if the last characters of `u` and `v` are the same.

Because we know all these distances we could just take the minimum between them.

## Algorithm
In each of the examples in the introduction we had pairs of strings for which
the Levenshtein distance was easily calculated. Calculating the Levenshtein
distance between arbitrary strings is a bit more involved. In this section we
will discuss various algorithms that calculates the Levenshtein distance between
strings, exploiting the patterns we noticed.

### Recursive
The patterns that we noticed relates the Levenshtein distance between string to
Levenshtein distances of smaller strings. This provides a hint to a recursive
algorithm.

```rust
pub fn recursive(u: &str, v: &str) -> u32 {
    if u.is_empty() { return v.len() as u32 }
    if v.is_empty() { return u.len() as u32 }

    let uc = u[(u.len() - 1) .. u.len()].chars().next();
    let vc = v[(v.len() - 1) .. v.len()].chars().next();
    let edit_cost = if uc == vc {0} else {1};

    let u_accent = &u[0..(u.len() - 1)];
    let v_accent = &v[0..(v.len() - 1)];

    minimum(
        recursive(&u_accent, &v) + 1,
        recursive(&u, &v_accent) + 1,
        recursive(&u_accent, &v_accent) + edit_cost
    )
}
```

## Exercises
1. Calculate the Levenshtein distance between `kangaroo` and `koala`.
2. Below you can find a matrix after the Levenshtein algorithm finished.
   Determine the necessary steps to transform `banner` into `hulk`.
   
     |   | `b` | `a` | `n` | `n` | `e` | `r`
     | 0 | 1   | 2   | 3   | 4   | 5   | 6 
 `h` | 1 | 1   | 2   | 3   | 4   | 5   | 6
 `u` | 2 | 2   | 2   | 3   | 4   | 5   | 6
 `l` | 3 | 3   | 3   | 3   | 4   | 5   | 6
 `k` | 4 | 4   | 4   | 4   | 4   | 5   | 6
 
[levenshtein]: https://en.wikipedia.org/wiki/Levenshtein_distance 
