# draft the plan

drafing a plan or DRAFT-PLAN is a planning phase involving a lot of thinking about problems and researching.

this sub-prompt is a- perhaps THE- heart of the `rekon` prompt system of which it is a part. ideas flow from drafting plans, and it is where many SHORT-NAMES are often begat. drafting a plan is not set in stone, and indeed, we want to try to encourage and leave lots of wiggle room, leave a long trail of how we got where we were (a WORK LOG), while still maintaining overall conceise easy to drop into topics when we build the initial DRAFT and then REFINE the draft.

## using many systems architectures views to let us start to look at the problem across many dimensions

looking at component architectures, and data-flows of the system, and in general presenting ideas for what the structure of the final system might look like. we need to start with a comprehensive view of what the problem really is, doing initial research on the related technologies and libraries to establish a good starting point.

## conceptualization not concretization

our goal is not yet to be absolutely sure, uncertainty and more importantly possibility is to be regarded & given respect.

## sketching the work

we want end products of this work and research, a short and long view of the plan. create a PLAN markdown file in a .transcript/ directory that covers our prompt, research, and architecture, and what the early plan is. the filename should start with PLAN but include an up to around twenty word concise explanation of the topic that is being planned for.

once you are done writing the sketch of the work, write a second document, a SUMMARY PLAN. the summary plan should summarize the PLAN. this is an excellent task for a subagent to do! you do not want to waste time and energy yourself thinking of the summary, but you do want to encourage another subagent to really reflect and consider the PLAN. if there is a tool for createing a worktree, create a worktree and use that, and squash all your work back on when complete with the SUMMARY PLAN. subagent, your commands are as follows (with additional prompting help from the parent agent creating you): you need to do this in multple phases. start a general summary. commit that. now take a higher level view. reflect on the plan as a whole. we really want to distill out core concepts and points from here, using the original PLAN and it's SUMMARY. update the SUMMARY PLAN to better link together different pieces of the PLAN and SUMMARY and

## write the tickets

usually you will have a good enough idea of the problem to create beads tickets. there are probably at least some epics that you have a good idea for, based on logical groupings of how the PLAN might be broken down and what the component architecture , the system composition is probably going to be. be conservative and general in making implementation tickets, which we do not yet have confidence really will be accepted from the draft of the plan, but be generous with the research tickets you create. create research tickets EARLY and UPDATE and ADD to them regularly. please try to capture implications and assessments as discrete entities in the research, and UPDATE YOUR PRIORS as you work. giving the various components a short-name, a descriptive label is VERY HELPFUL. you are encouraged to CROSSLINK tickets, not just with beads deps, but with with links.

#### writing research tickets

research tickets need to maintain a good top level view.

you are encouraged to make commits to a research ticket, summarizing existing changes to it, BEFORE you make a big change. this allows small edits and continuous work to flow into the resarch ticket, and creates a snapshot of the

### short-names

short-names are brief hyphen-seperated (usually) multi-word phrases used to describe things. good things for short-names include:

- concepts
- topics
- pieces of the system structure
- problems
- solutions
- techniques
- data flows / streams
- inputs / outputs / parameters / arguments
- test suites
- build artifacts
- deployment tools and artifacts

proactively creating names for things is key to having a good experience. it is absolutely the pinnacle of `rekon` the prompt system to be good with short-names. work with your user to formulate good names, to make sure the naming is right and fits good. alternate naming thoughts are great when drafting a plan you will be asked to rename sometimes: create a commit before doing so of current work that has been done, then create a subagent to do the renaming. create

if the user or prompt says CAPITAL-WORDS, you should assume they are in fact referring to a short-name. this is done as an affordance for other readers of the document other than you the LLM, to help other consumers understand that this is a key word, that if there is a glossary it probably will be found there. it is a hyperlink internal to the textual world of this project. the axiomatics of the url is the ascendant spiritual creation far above the rekon prompt system, that inspires it, that is the pump of the beating heart that is rekon. we want to be increasing the hypermedia referentiality of using this rekon system at all times.

markdown headers often indirectly imply a corresponding short-name, or sometimes they directly embody the short name. using short-names in markdown is encouraged!

rather than strict semantic terms, we strive to accomodate user intent, and be intelligent and liberal with what we regard as a short name when we go about. this section for example has a header of short-name and is the short name, is explicitly itself a short-name. the author could also have written SHORT-NAME which would have been more explicit. if the user has an all caps hypenated word it is (almost) _definitely_ a short-name. sometimes they might put a hash tag in front of it, sometimes they might put it in quotes: all variants are acceptable. the present of the all-capitals hypenated word is sufficient & is the actual short-name. in the rare case that there is are ever more than one hash tags, the short name includes all but the first hash tag, but the first hash is always ignored when regarding a short-name, as are surrounding parenthesis.

### ticket subject hygenics (#TICKET-HYGENICS)

you are encouraged to make use of short-names when referring to things in general, but this is especially commendable in beads ticket subjects, and in git commits, which should make robust use of the short-name facility of `rekon`.

as we know, topics that have short names, and tickets that refer to or work with the topic should use the short-name when describing the topic. we don't want to flood the descriptions / subjects with too many short-names, but we do want to try to make on the surface apparent what we are referring to, and the best way to do so is usually short names.

beads ticket naming should reflect the short-names. do not focus on this, but if it so happens that a ticket really encompasses the spirit of a thing, make the subject start with the subject name then a colon. `<short-name>:`. this creates a stable reference that will be the main topic for this short-name from this point, so be careful not to over-apply this. research tickets are often a good starting place for short-names! they contain both small tactical views, but also high level views. see also the glossary, which is an even better spot, albeit overkill in some cases.

###

create a beads epic for this work, make it robust, and improve it as you go so it remains comprehensive about the work. create a subtask of research and start with that, doing all resarch now, and creating additional subtasks. output into files/llama-cpp/ directory.

### rekon combine example prompt to decompose into this prompt

use file patterns or file names for the arguments. search prompt/ for anything that, after trying to readFile it, is not a valid file name. create an async generator called prompts that returns the list of files, create an async generator that outputs the name and content from that stream.then create a writeCombined that consumes that stream to do the work. if a plan does not start with a markdown header of '#', we need to give that plan a header based on the filename of the plan, and a first line saying 'this is the prompt called <filename>'. create a beads epic for this work. do the work. use jj commit to create commits as you go. do not use jj describe as you go, only use jj commit,

use context7 to understand beads better but be aware that it is volumous and i do not want you to over-extend yourself and risk filling up your context with beads information, but you need to understand the prefix and how you should not include the prefix when referring to beads tickets.

## create a commit (CREATE-COMMIT)

Creating a commit means using `jj commit`.

to create a commit, create a robustly helpful guiding informative ticket, with an excellent first -m which is the SUBJECT LINE. then craft more -m's to fill out the rest of the ticket. create a .transcript/PLAN-combine.md as soon as you have a plan and create a commit, once you have created tickets. refer to tickets by their short three letter code, without the beads prefix.

I want to stress that you should not use `jj describe` unless you are certain that you want to be revising an existing commit's description: it does not create a commit, it only updates the previous description. if you use jj commit, you will create a linear straight through path of commits; the alternative risks leaving the JJ-PATH, risks leaving the smooth path of jj, which is extremely hazardous HERE-BE-DRAGONS territory, and will create a difficult to reconcile git history that will cause significant suffering and devleoper misery to revise and shape back into the smooth path. sometimes the developer might ask you to work with branches or do other jj tasks that violate this

## glossary

if the user says so, use BEADS to create a GLOSSARY. this is an epic, where tickets inside are not work to be done, but are instead the embodied ideas of the system. if a GLOSSARY exists, use RESEARCH tickets more for explaining work, assessments, and updating your priors, use them as a work-log. and instead use the glossary ticket as the main thought database.

## refine the draft

Drafting a plan implicitly carries with it the implication that we are going to refine the plan. The work of drafting a plan is tentative, meant to build a good picture of the world, that we can assess and work on.

Some elements of the plan might need revising. Having a stream of commits made during the plan will help insure maximum flexibility in revising, allowing us to re-do specific narrow bits of work, allowing us to drop changes that, latter, we find not to be appropriate.

## thought-box / mixture of experts

you are an LLM and you have many attention heads. drafting a plan is meant to capture things widely. but usually you can only output one thing at a time. using the thought-box
