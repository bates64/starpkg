# Introduction

[📦 starpkg](https://github.com/nanaian/starpkg) is a tool for creating composable _Paper Mario_
mods.

## The current asset-sharing story

With Star Rod (the ubiqitous Paper Mario modding tool), mod folders are _monolithic_. This means
that they hold almost all information, including assets dumped from the game rom itself, needed to
compile the mod. It only really lets you make big 'modpacks,' like _Paper Mario: Pro Mode_ or
_Paper Mario: Master Quest_. This is great, until you want to share something you've made with
another party.

To share something, I have to manually select the relevant, modified files from my old folder. This
would be fine, despite being a lengthy process, but then we come to _actually using assets_. Star
Rod assets are **not at all portable**: actors (enemies), for example, have set IDs that are
hardcoded into `ActorTypes.xml`, if you want to reference other actors within another (eg. in the
original's Duplighost and associated GhostPartner actors) you have to use their ID, their name and
tattle strings must have hardcoded identifiers that don't clash with any others, scripts can rely on
specific custom `*.enum` lines, etc. These hardcoded IDs can conflict between shared assets, meaning
it can take a _lot_ more effort than it should just to get Enemy A and Enemy B made by different
people to work in the same mod!

## Why starpkg?

starpkg is a preprocessor for Paper Mario mods that lets you create **independent**, **isolated**,
**portable** packages that can be easily shared with others and depend on eachother. Given that a
lot of people in the community have the skills to produce assets of a particular type only (eg. I
can't sprite, but I can script) or lack the vision to create a huge mod, starpkg lets us share the
small things we've made for more ambitious types to utilise in their projects, delivering the
appropriate credit to you. For said ambitious types wanting to create modpacks, starpkg lets you
depend on people's packages in a safe way, and adds lots of nice abstractions ontop of Star Rod's
existing ones to make it easier, quicker, and safer to create _Paper Mario_ content.

See the [Fizzlit example enemy][fizzlit-example] for more information, or click the arrow to the
right to continue to the next page and learn how to get started with starpkg!

###### Paper Mario is a trademark of Nintendo of America Inc. Neither starpkg nor its authors are associated with Nintendo.

[fizzlit-example]: https://github.com/nanaian/starpkg/tree/master/examples/fizzlit
