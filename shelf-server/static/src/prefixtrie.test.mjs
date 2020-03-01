import test from "ava";
import PrefixTrie from "./prefixtrie.mjs";

test("empty prefix trie has no words", t => {
    const trie = new PrefixTrie();
    t.deepEqual(trie.beginsWith(""), new Set());
    t.deepEqual(trie.beginsWith("foo"), new Set());
});

test("we can look up a word we insert", t => {
    const trie = new PrefixTrie(["hello world"]);
    t.deepEqual(trie.beginsWith(""), new Set(["hello world"]));
    t.deepEqual(trie.beginsWith("h"), new Set(["hello world"]));
    t.deepEqual(trie.beginsWith("he"), new Set(["hello world"]));
    t.deepEqual(trie.beginsWith("hel"), new Set(["hello world"]));
    t.deepEqual(trie.beginsWith("hell"), new Set(["hello world"]));
    t.deepEqual(trie.beginsWith("hello world"), new Set(["hello world"]));
    t.deepEqual(trie.beginsWith("a"), new Set());
});

test("we can look multiple words", t => {
    const trie = new PrefixTrie(["hello world", "hello kitty", "foobar"]);
    t.deepEqual(trie.beginsWith(""), new Set(["hello world", "hello kitty", "foobar"]));
    t.deepEqual(trie.beginsWith("hello "), new Set(["hello world", "hello kitty"]));
    t.deepEqual(trie.beginsWith("hello world"), new Set(["hello world"]));
    t.deepEqual(trie.beginsWith("f"), new Set(["foobar"]));
    t.deepEqual(trie.beginsWith("a"), new Set());
});
