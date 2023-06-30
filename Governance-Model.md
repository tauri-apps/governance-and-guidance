# "Domains and Teams" governance model

_Previously called "Governance model v1.0"._

## Overview

<!-- ![Diagram](https://i.imgur.com/n1XWYxh.png) -->
<!-- ![Diagram](https://i.imgur.com/ZqWcRxn.png) -->
![Diagram](images/governance-diagram-detailed.svg)

<!-- https://www.figma.com/file/DVLmSm2wl93ZXYAMGDGZF8/Tauri-Governance?node-id=0-1&t=FvE2UiMG7uVDtL3a-0 -->

## Why

We're a growing project! And to keep up with everything that's happening, it's essential that people who want to help are empowered to do so. Reorganizing the working group is part of that.

Right now it can be unclear who's working on what, who to ask a particular question, and when it's OK to do something or if you need a review. So this structure aims to be:

- Easier to indicate your interests & expertise
- More obvious who can make decisions and where responsibilities are
- A clear structure to manage permissions

## Transitioning

The main priority for transitioning should be **continuity**.

- [ ] Phase 0: Mapping out current situation and desired domains
- [ ] Phase 1a: [Formalizing Domain Lead roles](/dyhez9LuQNyPpvcvmES8SA)
- [ ] Phase 1b: Appointing people with *current admin access* to corresponding Teams.
      *This is expected to be very messy, and people will be in many teams. Reflecting reality.*
- [ ] Phase 2: Appointing Domain Leads. Ensuring they have corresponding access too.
- [ ] Phase 3: Gradually limit permissions and let people move teams per their interest & expertise.

## Terminology

### Tauri Board

The Board is well defined by the statutes. But summarizing, they're the *decision making body* for anything in the project. That's a lot to stay on top of though, so the Board may delegate some of this.

### Working Group

- *The Working Group is...*

### Domains

The vast majority of issues or projects should be *owned* to one of the Domains. Examples of Domains are Community, Development, Governance and Operations.

For instance:
- "I have an idea for a tweet!" -> Community
- "There's a problem with the tauri build pipeline" -> Operations

Of course sometimes things overlap, in that case we still (arbitrarily) pick one domain as the owner. Though this shouldn't prevent anyone from contributing. See "Working Group members".

<details><summary><em>By the way, a great way to document and organize these arbitrary choices is through Teams.</em></summary>

(Uses concepts explained later. Come back later if it doesn't make sense.)

As an example, our Discord server as a whole could fit in several domains. Operations, Community, possibly even Governance.

Imagine a Team description like:
> Name: Discord team
> Domain: Operations
> 
> Handles everything related to our Discord server. Including the bots, moderation and how it's organized. Keep in mind Community is also affected, so please consult the Community Domain Leads as well on any big changes.

While it's totally arbitrary, it's unambiguous. The Discord Team manages it, they fall under the Operations domain, and the Operations Domain Leads are responsible. The Community domain is related, but not the domain that *owns* this aspect.

In practice, I would consider several more specific teams for Discord, but this illustrates how you could *assign* a complex topic to one domain through teams.

</details>

### Domain Leads

A key new role is the Domain Lead.

They are similar to Board Directors, but overseeing a particular domain.
**Domain Leads SHOULD be [formal roles](https://dracc.commonsconservancy.org/0035/#governance), and require signing [the Pledge](https://dracc.commonsconservancy.org/0016/)**.
Including as well, the Board may decide who to appoint or remove as Domain Leads.

Normally, they *don't do all the work*. But they're the stewards and main point of contact for the functioning, continuity and decision making within that domain.

They can handle a question like "should we automatically mirror our Tweets to Mastodon?"
And typically, they will have *admin access* to the tools and services in their domain.

In case the Domain Leads have difficulty making a decision on their own, they can consult other members of the working group, or escalate their question to the Tauri Board. For example with a controversial question like "Should we have a stricter moderation policy on Electron-bashing?" the Domain Leads may ask the Board for help in deciding.

The reverse is also true, should the Board be concerned about an aspect of that domain (ex: "Hey, we noticed the Reddit has become very toxic.") the Board should bring this to the attention of the Domain Leads and look for a solution.

### (Domain) Teams

Teams are small groups that have ownership over a very specific area or project. Teams are part of a particular domain, and very lightweight.

*An example:*
> The Discord Bot Team (part of the Operations domain) is in charge of the Discord Bot git repo. They have commit access, may review PRs and through CI/CD can deploy new versions of the bot on our Discord server. However they are not admins of the Discord server.

Domain Leads may create Teams, appoint or remove Team members and decide how those Teams should function. **They're the main way for Domain Leads (and others) to keep track of who's working on what.**

They're also an excellent entry-point for new contributors who are joining the Working Group to help out with a specific task they're interested in.

Becoming a Team member normally does not require formal roles / pledge signing. Instead the Domain Leads remain responsible. And because that is the case, privileges for Team members should be limited accordingly.

*Transition phase:*
Initially, continuity and bus-factor are more important than restricting access. So until we've rolled out this governance model more broadly, admin access by team members will probably be common. This should be phased out when that can be done in a non-disruptive way.


### Working Group members

*In the real world*, cleanly fitting a project into just one domain isn't realistic. Things will overlap and interact, no matter how you divide them. So the domains will not be configured as silos!

A member of the Working Group, has access to see what's happening in every domain. Though they can of course choose what aspects they care about and set notifications as they like, but the point is there aren't any unnecessary roadblocks between the domains.

There's no major responsibilities that come with being a Working Group member. They have access to more information, a cool badge, a few things they may be able to edit. So this needs to be handled appropriately. But the real fun starts in the Teams!

If there's a project they would like to tackle, an itch that needs scratching, something specific they're interested in, they can ask a Domain Lead to join or create a Team.

People can join multiple Teams from any domain. Again not putting up needless barriers between domains. And if someone is a Board Director, Domain Lead or Team member, they're automatically a Working Group member as well.

### CORE <sup>new</sup> definition

So you might notice, there's no Core in this model so far.
Yes, I suggest that Core becomes a *derived group*, which you don't join directly.

Instead CORE <sup>new</sup>, consists of:
- The Tauri Board
- All Domain Leads

The benefit is, it's much clearer what responsibilities are when you're a Director or Domain Lead. And lets people choose which domain(s) they want to be involved with.

Of course existing Core members are obvious candidates for the Domain Lead roles, so with this approach there shouldn't be a great change in who is in Core vs CORE <sup>new</sup>.

For all intents and purposes though, this is *just terminology*. And might be used for example to simplify public messaging "the Core team" or to give shiny Discord titles. But there's no description of what a CORE <sup>new</sup> member is or does, and it MUST NOT be used for access control. This is all handled by the Board Director and Domain Lead roles.

## Role kinds (recap)

Any one person can be in multiple roles.
In order of responsibility & privileges.

- **Board Director**, as per statutes.
- **Domain Lead**, as formalized roles.
- **Team member**, as defined by a Domain's teams.
- **WG member**, meaning anyone in the Working Group.
- `@everyone`, the general community and public.

---


<details><summary>Previous text example structure</summary>

_Note: teams are examples._

- **Tauri Board**
    - 3-7 **Directors**
- **Domain: Governance**
    - 2-3 **Leads** (ideally, fallback Directors)
    - RFC team
    - PM team
    - (...)
- **Domain: Operations**
    - 2-3 **Leads** (ideally, fallback Directors)
    - Email team
    - Padloc team
    - GitHub configuration team
    - CI/CD team
    - DNS team
    - (...)
- **Domain: Development**
    - 2-3 **Leads** (ideally, fallback Directors)
    - tauri-apps/tauri maintainer team
    - tauri-apps/tao maintainer team
    - tauri-apps/wry maintainer team
    - mobile maintainer team
    - (...)
- **Domain: Community**
    - 2-3 **Leads** (ideally, fallback Directors)
    - Mastodon team
    - Twitter team
    - YouTube content team
    - YouTube moderation team
    - Discord moderation team
    - Website team
    - Design team
    - (...)

Handy other reference: https://docs.google.com/spreadsheets/d/1D7ZfTKX6IzwROcp8XXUjHWI98HgA60bM7SQfPsZlbwc/edit#gid=0

</details>

<details><summary>Previous diagram</summary>

![](https://i.imgur.com/4yrKC4J.png)

</details>
