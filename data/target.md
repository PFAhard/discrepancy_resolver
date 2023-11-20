| [[N&#x2011;01](#01)] | NatSpec documentation for contract is missing | 4| 0|
| [[N&#x2011;02](#02)] | Names of `private`/`internal` functions should be prefixed with an underscore | 1| 0|
| [[N&#x2011;03](#03)] | Order of functions does not follow the Solidity Style Guide | 13| 0|
| [[N&#x2011;04](#04)] | Use of `override` is unnecessary | 2| 0|
| [[N&#x2011;05](#05)] | Custom errors should be used rather than `revert()`/`require()` | 12| 0|
| [[N&#x2011;06](#06)] | Large numeric literals should use underscores for readability | 6| 0|
| [[N&#x2011;07](#07)] | Assembly blocks should have extensive comments | 1| 0|
| [[N&#x2011;08](#08)] | Simplify complex require statements | 1| 0|
| [[N&#x2011;09](#09)] | Consider moving `msg.sender` checks to `modifier`s | 2| 0|
| [[N&#x2011;10](#10)] | Consider adding a block/deny-list | 2| 0|
| [[N&#x2011;11](#11)] | Constants/Immutables redefined elsewhere | 2| 0|
| [[N&#x2011;12](#12)] | Convert simple `if`-statements to ternary expressions | 1| 0|
| [[N&#x2011;13](#13)] | Events should be emitted before external calls | 8| 0|
| [[N&#x2011;14](#14)] | Events are emitted without the sender information | 3| 0|
| [[N&#x2011;15](#15)] | Event is missing `indexed` fields | 3| 0|
| [[N&#x2011;16](#16)] | Imports could be organized more systematically | 4| 0|
| [[N&#x2011;17](#17)] | @openzeppelin/contracts should be upgraded to a newer version | 6| 0|
| [[N&#x2011;18](#18)] | Lines are too long | 11| 0|
| [[N&#x2011;19](#19)] | Magic numbers should be replaced with constants | 14| 0|
| [[N&#x2011;20](#20)] | Memory-safe annotation preferred over comment variant | 1| 0|
| [[N&#x2011;21](#21)] | Use `@inheritdoc` for overridden functions | 2| 0|
| [[N&#x2011;22](#22)] | Contracts should have NatSpec `@author` tags | 4| 0|
| [[N&#x2011;23](#23)] | Contracts should have `@notice` tags | 4| 0|
| [[N&#x2011;24](#24)] | Contracts should have NatSpec `@title` tags | 4| 0|
| [[N&#x2011;25](#25)] | Events missing NatSpec `@param` tag | 12| 0|
| [[N&#x2011;26](#26)] | Event declarations should have NatSpec descriptions | 12| 0|
| [[N&#x2011;27](#27)] | Functions missing NatSpec `@param` tag | 7| 0|
| [[N&#x2011;28](#28)] | NatSpec documentation for function is missing | 5| 0|
| [[N&#x2011;29](#29)] | Functions missing NatSpec `@return` tag | 9| 0|
| [[N&#x2011;30](#30)] | State variables should have NatSpec descriptions | 6| 0|
| [[N&#x2011;31](#31)] | Contract name does not follow the Solidity Style Guide | 2| 0|
| [[N&#x2011;32](#32)] | Functions should be named in mixedCase style | 3| 0|
| [[N&#x2011;33](#33)] | Variable names for `immutable`s should use UPPER_CASE_WITH_UNDERSCORES | 4| 0|
| [[N&#x2011;34](#34)] | Missing zero address check in functions with address parameters | 3| 0|
| [[N&#x2011;35](#35)] | Constants should be put on the left side of comparisons | 11| 0|
| [[N&#x2011;36](#36)] | Large multiples of ten should use scientific notation | 4| 0|
| [[N&#x2011;37](#37)] | Non-interface files should use fixed compiler versions | 2| 0|
| [[N&#x2011;38](#38)] | Unused contract variables | 1| 0|
| [[N&#x2011;39](#39)] | Unused import | 1| 0|
| [[N&#x2011;40](#40)] | Consider using `delete` rather than assigning zero to clear values | 2| 0|
| [[N&#x2011;41](#41)] | Use the latest Solidity version | 4| 0|
| [[N&#x2011;42](#42)] | Named mappings are recommended | 10| 0|
| [[N&#x2011;43](#43)] | Whitespace in Expressions | 1| 0|
| [[N&#x2011;44](#44)] | Modifier declarations should have NatSpec descriptions | 1| 0|
| [[N&#x2011;45](#45)] | Contract functions should use an `interface` | 18| 0|
| [[N&#x2011;46](#46)] | Addresses shouldn't be hard-coded | 3| 0|
| [[N&#x2011;47](#47)] | Multiple mappings with same keys can be combined into a single struct mapping for readability | 4| 0|
| [[N&#x2011;48](#48)] | Control structures do not follow the Solidity Style Guide | 1| 0|
| [[N&#x2011;49](#49)] | Do not cache immutable variables | 2| 0|
| [[N&#x2011;50](#50)] | Missing event for critical changes | 1| 0|
| [[N&#x2011;51](#51)] | Duplicated `require()`/`revert()` checks should be refactored | 1| 0|
| [[N&#x2011;52](#52)] | Consider adding emergency-stop functionality | 3| 0|
| [[N&#x2011;53](#53)] | Avoid the use of sensitive terms | 23| 0|
| [[N&#x2011;54](#54)] | Contracts should have NatSpec `@dev` tags | 4| 0|
| [[N&#x2011;55](#55)] | Missing NatSpec `@dev` from event declaration | 12| 0|
| [[N&#x2011;56](#56)] | Missing NatSpec `@notice` from event declaration | 12| 0|
| [[N&#x2011;57](#57)] | Missing NatSpec `@dev` from function declaration | 23| 0|
| [[N&#x2011;58](#58)] | Missing NatSpec `@notice` from function declaration | 5| 0|
| [[N&#x2011;59](#59)] | Missing NatSpec `@dev` from modifier declaration | 1| 0|
| [[N&#x2011;60](#60)] | Missing NatSpec `@notice` from modifier declaration | 1| 0|
| [[N&#x2011;61](#61)] | Contracts should have full test coverage | 1| 0|
| [[N&#x2011;62](#62)] | Large or complicated code bases should implement invariant tests | 1| 0|
| [[N&#x2011;63](#63)] | Top-level declarations should be separated by at least two lines | 12| 0|
| [[N&#x2011;64](#64)] | Consider adding formal verification proofs | 4| 1000|
| [[N&#x2011;65](#65)] | Error messages should be descriptive, not cryptic | 6| 18|