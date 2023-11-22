# Extending

Substrait is a community project and requires consensus about new additions to the specification in order to maintain consistency.  The best way to get consensus is to discuss ideas.  The main ways to communicate are:

* Substrait Mailing List
* Substrait Slack
* Community Meeting

## Minor changes

Simple changes like typos and bug fixes do not require as much effort.  [File an issue](https://github.com/substrait-io/substrait/issues) or [send a PR](https://github.com/substrait-io/substrait/pulls) and we can discuss it there.

## Complex changes

For complex features it is useful to discuss the change first.  It will be useful to gather some background information to help get everyone on the same page.

### Outline the issue

#### Language

Every engine has its own terminology.  Every Spark user probably knows what an "attribute" is.  Velox users will know what a "RowVector" means.  Etc.  However, Substrait is used by people that come from a variety of backgrounds and you should generally assume that its users do not know anything about your own implementation.  As a result, all PRs and discussion should endeavor to use Substrait terminology wherever possible.

#### Motivation

What problems does this relation solve?  If it is a more logical relation then how does it allow users to express new capabilities?  If it is more of an internal relation then how does it map to existing logical relations?  How is it different than other existing relations?  Why do we need this?

#### Examples

Provide example input and output for the relation.  Show example plans.  Try and motivate your examples, as best as possible, with something that looks like a real world problem.  These will go a long ways towards helping others understand the purpose of a relation.

#### Alternatives

Discuss what alternatives are out there.  Are there other ways to achieve similar results?  Do some systems handle this problem differently?

### Survey existing implementation

It's unlikely that this is the first time that this has been done.  Figuring out

### Prototype the feature

Novel approaches should be implemented as an extension first.  

### Substrait design principles 

Substrait is designed around interoperability so a feature only used by a single system may not be accepted.  But don't dispair!  Substrait has a highly developed extension system for this express purpose.

### You don't have to do it alone

If you are hoping to add a feature and these criteria seem intimidating then feel free to start a mailing list discussion before you have all the information and ask for help.  Investigating other implementations, in particular, is something that can be quite difficult to do on your own.
