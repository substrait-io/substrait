---
title: Governance
---

# Substrait Project Governance

The Substrait project is run by volunteers in a collaborative and open way. Its governance is inspired by the Apache Software Foundation. In most cases, people familiar with the ASF model can work with Substrait in the same way. The biggest differences between the models are:

- Substrait does not have a separate infrastructure governing body that gatekeeps the adoption of new developer tools and technologies.
- Substrait Management Committee (SMC) members are responsible for recognizing the corporate relationship of its members and ensuring diverse representation and corporate independence.
- Substrait does not condone private mailing lists. All project business should be discussed in public The only exceptions to this are security escalations (security@substrait.io) and harassment (harassment@substrait.io).
- Substrait has an automated continuous release process with no formal voting process per release.

More details about concrete things Substrait looks to avoid can be found below.

## The Substrait Project

The Substrait project consists of the code and repositories that reside in the [substrait-io GitHub organization](https://github.com/substrait-io), the [Substrait.io website](https://substrait.io), the [Substrait mailing list](https://groups.google.com/g/substrait), MS-hosted teams community calls and the [Substrait Slack workspace]({{versions.slackinvitelink}}). (All are open to everyone and recordings/transcripts are made where technology supports it.)

## Substrait Volunteers

We recognize four groups of individuals related to the project.

### User

A user is someone who uses Substrait. They may contribute to Substrait by providing feedback to developers in the form of bug reports and feature suggestions. Users participate in the Substrait community by helping other users on mailing lists and user support forums.

### Contributors

A contributor is a user who contributes to the project in the form of code or documentation. They take extra steps to participate in the project (loosely defined as the set of repositories under the github substrait-io organization) , are active on the developer mailing list, participate in discussions, and provide patches, documentation, suggestions, and criticism.

### Committer

A committer is a developer who has write access to the code repositories and has a signed [Contributor License Agreement (CLA)](https://cla-assistant.io/substrait-io/substrait) on file. Not needing to depend on other people to make patches to the code or documentation, they are actually making short-term decisions for the project. The SMC can (even tacitly) agree and approve the changes into permanency, or they can reject them. Remember that the SMC makes the decisions, not the individual committers.

### SMC Member

A SMC member is a committer who was elected due to merit for the evolution of the project. They have write access to the code repository, the right to cast binding votes on all proposals on community-related decisions,the right to propose other active contributors for committership, and the right to invite active committers to the SMC. The SMC as a whole is the entity that controls the project, nobody else. They are responsible for the continued shaping of this governance model.

## Substrait Management and Collaboration

The Substrait project is managed using a collaborative, consensus-based process. We do not have a hierarchical structure; rather, different groups of contributors have different rights and responsibilities in the organization.

## Communication

Communication must be done via mailing lists, Slack, and/or Github. Communication is always done publicly. There are no private lists and all decisions related to the project are made in public. Communication is frequently done asynchronously since members of the community are distributed across many time zones.

## Substrait Management Committee

The Substrait Management Committee is responsible for the active management of Substrait. The main role of the SMC is to further the long-term development and health of the community as a whole, and to ensure that balanced and wide scale peer review and collaboration takes place. As part of this, the SMC is the primary approver of specification changes, ensuring that proposed changes represent a balanced and thorough examination of possibilities. This doesn’t mean that the SMC has to be involved in the minutiae of a particular specification change but should always shepard a healthy process around specification changes.

## Substrait Voting Process

Because one of the fundamental aspects of accomplishing things is doing so by consensus, we need a way to tell whether we have reached consensus. We do this by voting. There are several different types of voting. In all cases, it is recommended that all community members vote. The number of binding votes required to move forward and the community members who have “binding” votes differs depending on the type of proposal made. In all cases, a veto of a binding voter results in an inability to move forward.

The rules require that a community member registering a negative vote must include an alternative proposal or a detailed explanation of the reasons for the negative vote. The community then tries to gather consensus on an alternative proposal that can resolve the issue. In the great majority of cases, the concerns leading to the negative vote can be addressed. This process is called "consensus gathering" and we consider it a very important indication of a healthy community.

|                                                                                                           | +1 votes required          | Binding voters | Voting Location |
| --------------------------------------------------------------------------------------------------------- | -------------------------- | -------------- | --------------- |
| Process/Governance modifications & actions. This includes promoting new contributors to committer or SMC. | 3                          | SMC            | Mailing List    |
| Format/Specification Modifications (including breaking extension changes)                                 | 2                          | SMC            | Github PR       |
| Non-breaking function introductions                                                                       | 1 (not including proposer) | Committers     | Github PR       |
| Non-breaking extension additions & non-format code modifications                                          | 1 (not including proposer) | Committers     | Github PR       |

### Review-Then-Commit

Substrait follows a review-then-commit policy. This requires that all changes receive consensus approval before being committed to the code base. The specific vote requirements follow the table above.

### Expressing Votes

The voting process may seem more than a little weird if you've never encountered it before. Votes are represented as numbers between -1 and +1, with '-1' meaning 'no' and '+1' meaning 'yes.'

The in-between values indicate how strongly the voting individual feels. Here are some examples of fractional votes and what the voter might be communicating with them:

- +0: 'I don't feel strongly about it, but I'm okay with this.'
- -0: 'I won't get in the way, but I'd rather we didn't do this.'
- -0.5: 'I don't like this idea, but I can't find any rational justification for my feelings.'
- ++1: 'Wow! I like this! Let's do it!'
- -0.9: 'I really don't like this, but I'm not going to stand in the way if everyone else wants to go ahead with it.'
- +0.9: 'This is a cool idea and I like it, but I don't have time/the skills necessary to help out.'

### Votes on Code Modification

For code-modification votes, +1 votes (review approvals in Github are considered equivalent to a +1) are in favor of the proposal, but -1 votes are vetoes and kill the proposal dead until all vetoers withdraw their -1 votes.

### Vetoes

A -1 (or an unaddressed PR request for changes) vote by a qualified voter stops a code-modification proposal in its tracks. This constitutes a veto, and it cannot be overruled nor overridden by anyone. Vetoes stand until and unless the individual withdraws their veto.

To prevent vetoes from being used capriciously, the voter must provide with the veto a technical or community justification showing why the change is bad.

## Why do we vote?

Votes help us to openly resolve conflicts. Without a process, people tend to avoid conflict and thrash around. Votes help to make sure we do the hard work of resolving the conflict.

## Substrait is non-commercial but commercially-aware

Substrait’s mission is to produce software for the public good. All Substrait software is always available for free, and solely under the Apache License.

We’re happy to have third parties, including for-profit corporations, take our software and use it for their own purposes. However it is important in these cases to ensure that the third party does not misuse the brand and reputation of the Substrait project for its own purposes. It is important for the longevity and community health of Substrait that the community gets the appropriate credit for producing freely available software.

The SMC actively track the corporate allegiances of community members and strives to ensure influence around any particular aspect of the project isn’t overly skewed towards a single corporate entity.

## Substrait Trademark

The SMC is responsible for protecting the Substrait name and brand. TBD what action is taken to support this.

## Project Roster

### Substrait Management Committee (SMC)

{{ read_yaml("./data/smc.yaml") }}

### Substrait Committers

{{ read_yaml("./data/committers.yaml") }}

## Additional detail about differences from ASF

Corporate Awareness: The ASF takes a [blind-eye](https://www.apache.org/foundation/how-it-works.html#hats) approach that has proven to be too slow to correct corporate influence which has substantially undermined many OSS projects. In contrast, Substrait SMC members are responsible for identifying corporate risks and over-representation and adjusting inclusion in the project based on that (limiting committership, SMC membership, etc). Each member of the SMC shares responsibility to expand the community and seek out corporate diversity.

Infrastructure: The ASF shows its age wrt to infrastructure, having been originally built on SVN. Some examples of requirements that Substrait is eschewing that exist in ASF include: custom git infrastructure, release process that is manual, project external gatekeeping around the use of new tools/technologies.
