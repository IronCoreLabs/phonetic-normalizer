# Phonetic Normalizer

There are a number of algorithms intended to make it easier to match words even when the words are under different spellings. Most of these algorithms, like Soundex and Caverphone focus on names. Metaphone and Double Metaphone and Eudex are all algorithms that are meant to work on dictionary words instead of just on names. These are the most promising existing algorithms for our purpose.

Our purpose is to store and index a normalized version of a word such that we accurately find it even when looking up the same word using different spellings. The produced word need not be readable or phonetic or anything. It could even be a number.  Ideally, it will do a good job of matching variations of names as well as dictionary words and, most importantly, it should work well even if we are matching hashes of words. In other words, we should not have to rely on substring matches at all.

In my testing of the above mentioned algorithms and a few others, I was disappointed by the results. For example, metaphone oversimplifies words and creates matches where there shouldn't be any. For example, the words `color`, `colour`, and `cooler` all reduce to `KLR`.  I want the first two words to be considered the same, but `cooler` has very different sounds and is pretty unlikely to be a misspelling.

Eudex seemed promising if we shift off the low bits. It produces a "hash" of sorts that can be used to compare how similar two words are. The closer the produced numbers to each other, the more similar they are.  Unfortunately, it doesn't well deal with misspellings of names. For example, `catherine` and `kathryn` are considered very different words even though they sound the same. In other word tests, I had a hard time tuning the bit shifting so misspelled words were identical and different words weren't.

In the end I was inspired by the Caverphone algorithm and its approach, but I wanted to make it more general for dictionary words.  In particular, my goal was to have no collisions when combing over a dictionary unless two words were either homonyms or were essentially the same word with different endings (like a word with and without a `-ed` suffix). I wanted to properly identify most if not all of a [common misspellings](https://www.lexico.com/grammar/common-misspellings) list and most or all of the [british/english spelling disagreements](http://www.tysto.com/uk-us-spelling-list.html). And I wanted to make the algorithm efficient so it wouldn't appreciably slow down indexing of documents. This meant constraining allocations of strings and doing as few passes over a word as possible.

I think I've succeeded reasonably well in building an algorithm that, for English at least, has a very low false positive rate (essentially zero from my testing) and a very strong ability to detect when two words are either homonyms or misspellings of each other.

Here are some of the words that will match:

| Word  | Alternate | Phonetically Normalized Version |
| ----- | --------- | --------------------------------|
| cough | coff | kof |
| piece | peace | peese |
| mist | missed | mist |
| phone | fone | fone |
| phony | fony | fony |
| accomodate | accommodate | acomotite |
| achieve | acheive | asheeve |
| apparent | apparant | aprent |
| basically | basicly | basicly |
| argument | arguement | **agument** * |
| definitely | definately | definatly |
| fourty | forty | forty |
| further | farther | frthr |
| gist | jist | gist |
| byte | bite | bite |
| siege | seige | seege |
| sense | sence | sense |
| consonant | consonent | konsonent |
| shaq | shack | **sak** * |
| gnat | nat | nat |
| knight | night | nite |
| night | nite | nite |
| knit | nit | nit |
| gnaw | naw | naw |
| natural | nateral | natrl |
| wherever | whereever | wrevr |
| color | colour | kolor |
| accessorise | accessorize | asesorise |
| abhominable | abominable | apominaple |
| curiousity | curiosity | kriosity |
| aerogramme | aerogram | arogram |
| almanack | almanac | almenak |
| anaemia | anemia | anemia |
| archaeology | archeology | asheology |
| behavioural | behavioral | beaviorl |
| cancellation | cancelation | kenselation |

\* These changed when I did my last round of optimizations. These should be `argument` and `shak`. Need to fix.

And these words do not match:

| Word1 | Word2 |
| ----- | ----- |
| at | ate |
| color | cooler |
| phony | phone
| John | gone
| precede | preset
| rupert | robert