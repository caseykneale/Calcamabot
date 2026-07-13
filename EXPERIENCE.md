# Experience Using Agents for This Project
As a continuation of my [study on what is gained/lost in a single LLM exchange](https://github.com/caseykneale/self_study_single_LLM_exchange) this project is geared at studying myself using an agentic harness to do software development. I chose to revisit a project that took me a few hours to do many years ago which is a CLI calculator. It was called [`Calcamabob`](https://github.com/caseykneale/Calcamabob). 

`Calcamabob`` wasn't a serious test of my abilities at the time. I'm not being shy about it. It wasn't that hard because there were awesome examples online that I learned from while doing it. I simply had never written parser before and figured I should. I was always curious to add more to that project but there was never a purpose to do so. I have other tools that calculate well readily at my disposal. Because that project was on the beaten path for an LLM I decided why not see how it would go recreating it with an agentic workflow. It could be a good exercise to study myself and the tooling in the process.

## Experiment Design
The purpose of this exercise was to create an advanced CLI calculator to replace some of my usage of other tools using a syntax I like using agentic AI programming tools. The task was time boxed to 5 days of off-and-on agentic programming while I worked on other tasks to see what I could create and maintain with local models. Note: This is **not** 40 hours of work or even a near equivalent. This is more like I wrote a plan, made lunch, did something else and checked in 3 hours later. The total time invested ignoring this human write up, and the compute time, was something like 6 hours?

## Questions I wanted to Answer
- Can local models build interesting green-field things?
- What is the experience like doing that? Did I learn something? Did it get stuck?
- What is the over-all quality of the code?

## Tools used
- LittleCoder
- Visual Studio IDE (to read code and diffs)
- LlamaCpp's `llama-server`
- deepreinforce-ai/Ornith-1.0-35B-GGUF
- Docker (I don't run agents directly on my machine)

## Project Evolution
- Phase 1: Foundation & Parsing
  - Implement a Pratt parser for basic mathematics.
  - Add transcendental functions and mathematical constants.
- Phase 2: Linear Algebra & Advanced Types
  - Support for vectors and matrices (custom notation).
  - Introduction of complex numbers and complex matrix operations.
  - Implementation of a broadcast operator.
- Phase 3: UX, Refinement & Export
  - Improved transcendental function support for complex numbers.
  - Implementation of a matrix/complex matrix pretty printer.
  - CSV export capabilities and final code cleanup.


## What went well
A lot went well. The agent more or less 2-shotted the original calcamabob projects goals. After that it was up to me to come up with features I actually wanted and how I wanted them done. From what I can tell the "final" result does what I asked of it. It didn't come super easy but I also didn't invest much into it at all.

One thing I really like is when the parser or the math fails it does throw mostly meaningful errors so the user can catch issues. Matrix dimensions don't match - nice! Or missing a bracket - oops! It doesn't say where the issues happened but that's okay. I also didn't ask for that.

Probably the most important thing is. It handles math I can't do in my head, and runs pretty fast. I'm mostly happy with the complex number support.

## What didn't go well
In this calculator I specifically wanted the following notation:
```
[[1,2],[3,4]] = 
1  3
2  4
```
Which would mean:
```
[[1,2],[3,4]]*[[1,1],[2,2]] = 
4  8
6  12
```

The agent fought me on this notation. It is somewhat unique in that it's a column major matrix construction notation, but that's typically how I construct matrices in Julia. It's how my brain works. Regardless despite that the matrices were written to be column major, the agent kept wanting the math to be done as if the matrices row major. It kept trying to force the above example calculation to be:

```
[[1,2],[3,4]]*[[1,1],[2,2]] = 
  5   5
 11  11
```

Needless to say many cycles were spent trying to steer it to this understanding, watching it loop, say "the user is confused" and make messes. I tried making smaller plans, larger ones, 'you figure it out' and serious hand-holding prompts. Almost all of these lead to a struggle.

Pretty printing was also a struggle. Despite describing very clearly what I wanted done, because of my notation choice it faught me. It kept transposing complex matrix results, and not real valued ones. Or the other way around. Despite explaining very clearly what was happening. This lead me to my biggest technical nuisance.

Although I didn't invest much time into it, I was specific in that I wanted complex matrices to have the same representation as real valued ones to reduce the code complexity. I laid out some ways to do it with traits, but alas, none landed. The Agent made and maintained 2 matrix types to support either complex or real valued matrices. I don't mind some duplication, especially for a project of this size, but I did a little more than hinting at a design which should have cut the code duplication in half. But I ran out of artificial time.

## Answers to my initial questions

### Can local models build interesting things?
Yes small local models can build interesting things. Definitely. This project plays to a coding models strengths when using Rust though. A ton of projects for parsers and the like exist in the rust ecosystem. So this really gave a coding model it's best chance of success. Who knows maybe even the original calcamabob project was in the training data for this model. Probably not, but you never know. 

**Grade: 90%**

### What is the experience like doing that?

| Subject | Score (0-10) | Summary |
| :--- | :--- | :--- |
| Effort | 10 | I was able to put in very litle in to get something |
| Understanding | 4 | I put very little into the code, so I don't understand or appreciate all of the decisions. |
| Performance | 7 | It didn't get in my way but it could have been faster |
| User experience | 5 | I was gaslit numerous times, and overly celebrated in others |

I didn't learn much. I wouldn't have learned more had I of not written a pratt parser before either. That part of the code worked well, and that's actually a bummer because I hardly saw the code after a quick review. If something went wrong there 1 month from now after adding a new feature I'd have to figure it all out and it would likely be harder than if I wrote it myself.

It wasn't light speed, sometimes when the model got stuck a few hours were spent looping until the harness bailed out and I came back to my computer. But for set it and forget it while doing something else, it was decent. The model getting stuck a bit wasn't a major penalty for me.

What I did find frustrating was being gaslit by a coding model that can't always execute what was asked of it. I appreciated the models skeptic replies to some extent, but when I had to write statements like `The user is not confused, this is required`. Yick. I knew what I wanted wasn't wildly idiomatic but it was what I wanted and I said so in no uncertain terms with examples and test cases. Like usual with LLM's the road well travelled is a lot easier than one of equivalent complexity that is less travelled. 

**Grade: 65%**

### What is the over-all quality of the code?

#### Quick human evaluation of each file
| Filename | Score (0-10) | Summary |
| :--- | :--- | :--- |
| ast.rs | 9 | Documentation could be better code is fine. |
| eval.rs  | 7.5 | Some of the code is repetitious and there's a smell around 2 types of matrices (Complex vs Matrix) |
| export.rs | 9 | I would have done it slightly differently but OK given what I asked. |
| function.rs | 9 | I may prefer the function names be organized as constants or controlled by an enum or something but it's fine for these purposes. |
| lexer.rs | 9 | Some small abstractions could go a long way to making this more legible. Pretty reasonable though. |
| lib.rs | 10 | I like that it added integration tests. It's a tiny file sure 10 |
| main.rs | 10 | This is more or less what I would expect. |
| parser.rs | 9 | It's a Pratt parser alright. Surprised there are no tests here? |
| tests | 9 | Some weird comments, but more or less I could work with these tests despite not loving their organization. |
| value.rs | 8 | Most of the code is fine but there is that 2 matrix type smell here and little reuse. I feel like there's a way to clean up the Display trait. |
| eval/matrix.rs | 6 | I don't like there being 2 matrix types there could be 1 type. it's a pile of longish public functions in another directory. I don't like abstraction for the sake of it, but there are functions here that should exist that don't. Like converting column to row major, etc.  |
| eval/range.rs | 9 | This is fine, but a single comment about edge cases would go a long way. |
| eval/scalar.rs | 7 | I'm kind of bummed about this one because I asked the harness multiple times not to use a macro here but it did it anyways. It really makes the code smell about complex vs real valued collections louder. |

**Average Score over files** : 86%

**Subjective Overall score by me** : 70% 

It's OK. I am not thrilled about it but its not that bad. I could improve the code more by doing a few more cycles but ran out of time. I personally don't feel like working on this more. Not because its terrible. Moreso because I'm not really interested in the project and to make it nice it would take an investment of time I don't want to spend. With more time it would be better, but, I can only spend so much time on experiments like this.

Again the easy stuff was easy. Lot's of files scored 9's or 10's, the easy code is more or less fine. Most of it could be cleaned up pretty quick with or without an agent. The trickier stuff that probably matters more would take considerably more time. 

## Overall
**Average Score: 78**

I could take this space to summarize everything I've said above. I'd rather synthesize and share some more meaningful things.

- Because the agent faught me to do the things it had seen before, I **almost compromised the design**. Maybe I should have, that's debatable. But if this was a major feature for a new product, would someone buckle and go into normalcy to not fight with the agent?
- This is a **small coding model running on a computer with hardware mostly from before 2020. It was able to offer a meaningful agentic programming environment** with a human in the loop! 2 years ago this wasn't possible at all.
- Bigger models may be better at inference but **if I threw code review out of my workflow it would have been a complete disaster**. Maybe for larger models it's more like "a small disaster" but I cannot understand people suggesting to do this right now even with frontier models. The amount of times tests passed and the happy path was fine but something I required was done differently than planned was nowhere near 0 even for this small green field project.
- The `experience` section recieved a bad grade. That's not because small models stink. There were times were I was slightly impressed. The downsides here are the same downsides that I've had with larger models. The slower results, and more touch points weren't an issue, they were almost nice. I thought about the project more despite this all being back burner asynchronous work. **The workflow itself was what failed it**. With a better model maybe it would have gotten a 70%, but maybe even a 50%. Why? Because I may have thought about the project less and understood it at an even shallower level. **In some ways its better to have a visibly less trust worthy coding model**.

## Subjective

I like the way small model coding is heading to some extent. I like being able to offload boring boiler plate tasks while focussing on the technical challenges of my hobby projects. With small hobby projects its fine to lick the frosting off the cake and leave the cake behind. This project was more or less something I didn't want to work on so it was interesting to do the whole thing this way. No pure frosting diet for me this week. Which is good. I don't really like frosting... Its just an example.

I could have done it better, but it would have taken more time than I afforded. Most of the code issues could probably be solved with a few more cycles with the agent. I guess if the goal was to `get something` then this is a win overall? At the same time its not something I feel like celebrating. I felt kind of ashamed putting code like this out in the public. Thats why I didn't even put my name or a license on it... Maybe I will if I get around to cleaning it up, but not for now.

___

### Note On Scores Used
I use numerical scores that range from `0` to `10` a bit in this write up. For technical work my view is the following:
 - `1-2`: Unreadable and techncially wrong
 - `3-4`: Unreadable or technically wrong
 - `5-6`: Has technical issues and is difficult to read
 - `7-8`: Notable issues and or notably difficult to maintain
 - `9`: Small gripes
 - `10`: I likely wouldn't change it if I was given free-range to improve it and there was other things to do. 

Maybe some peoples `10` would be "amazing code". In my experience you almost never get "amazing code" from an LLM when its used this way. Irrespective of the model or the workflow. It's always a bit sloppy even after cleaning things up. So I am rating a 10 at what I think the tool is capable of in it's best case scenario.
