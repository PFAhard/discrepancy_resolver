## Medium Risk Issues

|Index|Issue|Instances|
|:---:|:---|:---:|
| [M-01](#M-01) | [Use `_safeMint` instead of `_mint` for `ERC721`](#M-01) | 2 |

2 instances over 1 issues

---

## Low Risk Issues

|Index|Issue|Instances|
|:---:|:---|:---:|
| [L-01](#L-01) | [Functions calling contracts/addresses with transfer hooks are missing reentrancy guards](#L-01) | 13 |
| [L-02](#L-02) | [Loss of precision](#L-02) | 8 |
| [L-03](#L-03) | [Privileged functions can create points of failure](#L-03) | 7 |
| [L-04](#L-04) | [`onlyOwner` functions not accessible if owner renounces ownership](#L-04) | 5 |
| [L-05](#L-05) | [Code does not follow the best practice of check-effects-interaction](#L-05) | 5 |
| [L-06](#L-06) | [Prevent division by 0](#L-06) | 2 |
| [L-07](#L-07) | [Use of `tx.origin` is unsafe in almost every context](#L-07) | 2 |
| [L-08](#L-08) | [Large assembly blocks should have extensive comments](#L-08) | 1 |
| [L-09](#L-09) | [Missing checks for `address(0)` when assigning values to address state variables](#L-09) | 1 |

44 instances over 9 issues

---

## NonCritical Risk Issues

|Index|Issue|Instances|
|:---:|:---|:---:|
| [N-01](#N-01) | [Incomplete NatSpec `@return` from function declaration](#N-01) | 27 |
| [N-02](#N-02) | [`Constants` in comparisons should appear on the left side](#N-02) | 25 |
| [N-03](#N-03) | [Missing NatSpec `@dev` from function declaration](#N-03) | 23 |
| [N-04](#N-04) | [`constants` should be defined rather than using magic numbers](#N-04) | 22 |
| [N-05](#N-05) | [Contract functions should use an `interface`](#N-05) | 18 |
| [N-06](#N-06) | [Custom `error` should be used rather than `require`/`assert`](#N-06) | 12 |
| [N-07](#N-07) | [Event declarations should have NatSpec descriptions](#N-07) | 12 |
| [N-08](#N-08) | [Missing NatSpec from event declaration](#N-08) | 12 |
| [N-09](#N-09) | [Missing NatSpec `@dev` from event declaration](#N-09) | 12 |
| [N-10](#N-10) | [Missing NatSpec `@notice` from event declaration](#N-10) | 12 |
| [N-11](#N-11) | [Missing NatSpec `@param` from event declaration](#N-11) | 12 |
| [N-12](#N-12) | [Event is missing `indexed` field](#N-12) | 11 |
| [N-13](#N-13) | [Using named parameters in `mapping` is best practice](#N-13) | 8 |
| [N-14](#N-14) | [Events may be emitted out of order due to reentrancy](#N-14) | 8 |
| [N-15](#N-15) | [Missing NatSpec `@param` from function declaration](#N-15) | 8 |
| [N-16](#N-16) | [State variables should include comments](#N-16) | 6 |
| [N-17](#N-17) | [Duplicated `require/if` statements should be refactored](#N-17) | 5 |
| [N-18](#N-18) | [Function declarations should have NatSpec descriptions](#N-18) | 5 |
| [N-19](#N-19) | [Missing NatSpec from function definition](#N-19) | 5 |
| [N-20](#N-20) | [Missing NatSpec `@notice` from function declaration](#N-20) | 5 |
| [N-21](#N-21) | [Large multiples of ten should use scientific notation](#N-21) | 4 |
| [N-22](#N-22) | [Missing NatSpec from contract declarations](#N-22) | 4 |
| [N-23](#N-23) | [Missing NatSpec `@author` from contract declaration](#N-23) | 4 |
| [N-24](#N-24) | [Missing NatSpec `@dev` from contract declaration](#N-24) | 4 |
| [N-25](#N-25) | [Missing NatSpec `@notice` from contract declaration](#N-25) | 4 |
| [N-26](#N-26) | [Missing NatSpec `@title` from contract declaration](#N-26) | 4 |
| [N-27](#N-27) | [State variables not capped at reasonable values](#N-27) | 4 |
| [N-28](#N-28) | [Unused contract variables](#N-28) | 3 |
| [N-29](#N-29) | [`constructor` missing zero address check](#N-29) | 3 |
| [N-30](#N-30) | [Complex math should be split into multiple steps](#N-30) | 3 |
| [N-31](#N-31) | [Function state mutability can be restricted to view](#N-31) | 3 |
| [N-32](#N-32) | [Consider using `delete` rather than assigning zero/false to clear values](#N-32) | 2 |
| [N-33](#N-33) | [Consider moving `msg.sender` checks to a common authorization `modifier`](#N-33) | 2 |
| [N-34](#N-34) | [Use of `override` is unnecessary](#N-34) | 2 |
| [N-35](#N-35) | [Missing event and or timelock for critical parameter change](#N-35) | 2 |
| [N-36](#N-36) | [Consider adding a block/deny-list](#N-36) | 2 |
| [N-37](#N-37) | [Functions which are either private or internal should have a preceding _ in their name](#N-37) | 1 |
| [N-38](#N-38) | [Emits without msg.sender parameter](#N-38) | 1 |
| [N-39](#N-39) | [`if`-statement can be converted to a ternary](#N-39) | 1 |
| [N-40](#N-40) | [External calls in an un-bounded `for`-loop may result in a DOS](#N-40) | 1 |
| [N-41](#N-41) | [Consider implementing two-step procedure for updating protocol addresses](#N-41) | 1 |
| [N-42](#N-42) | [Events that mark critical parameter changes should contain both the old and the new value](#N-42) | 1 |
| [N-43](#N-43) | [Memory-safe annotation preferred over comment variant](#N-43) | 1 |
| [N-44](#N-44) | [Unused function parameter](#N-44) | 1 |
| [N-45](#N-45) | [Modifier declarations should have NatSpec descriptions](#N-45) | 1 |
| [N-46](#N-46) | [Missing NatSpec from modifiers definitions](#N-46) | 1 |
| [N-47](#N-47) | [Missing NatSpec `@dev` from modifier declaration](#N-47) | 1 |
| [N-48](#N-48) | [Missing NatSpec `@notice` from modifier declaration](#N-48) | 1 |
| [N-49](#N-49) | [Missing NatSpec `@param` from modifier declaration](#N-49) | 1 |
| [N-50](#N-50) | [Contracts should have full test coverage](#N-50) | 1 |
| [N-51](#N-51) | [Large or complicated code bases should implement invariant tests](#N-51) | 1 |
| [N-52](#N-52) | [Codebase should implement formal verification testing](#N-52) | 1 |

314 instances over 52 issues

---

## Gas Risk Issues

|Index|Issue|Instances|
|:---:|:---|:---:|
| [G-01](#G-01) | [State variables should be cached in stack variables rather than re-reading them from storage](#G-01) | 21 |
| [G-02](#G-02) | [Avoid Unnecessary Public Variables](#G-02) | 18 |
| [G-03](#G-03) | [Consider Using Solady's Gas Optimized Lib for Math](#G-03) | 17 |
| [G-04](#G-04) | [Optimize External Calls with Assembly for Memory Efficiency](#G-04) | 14 |
| [G-05](#G-05) | [Consider Caching Multiple Accesses to Mappings/Arrays](#G-05) | 13 |
| [G-06](#G-06) | [Use Custom Errors](#G-06) | 12 |
| [G-07](#G-07) | [Consider using `bytes32` rather than a `string`](#G-07) | 12 |
| [G-08](#G-08) | [Use Assembly for Efficient Event Emission](#G-08) | 12 |
| [G-09](#G-09) | [Stack variable used as a cheaper cache for a state variable is only used once](#G-09) | 9 |
| [G-10](#G-10) | [Optimize Zero Checks Using Assembly](#G-10) | 9 |
| [G-11](#G-11) | [Add `unchecked` blocks for divisions where the operands cannot overflow](#G-11) | 8 |
| [G-12](#G-12) | [Trade-offs Between Modifiers and Internal Functions](#G-12) | 7 |
| [G-13](#G-13) | [Multiple mappings can be replaced with a single struct mapping](#G-13) | 6 |
| [G-14](#G-14) | [Functions guaranteed to revert when called by normal users can be marked `payable`](#G-14) | 5 |
| [G-15](#G-15) | [`>=` costs less gas than `>`](#G-15) | 5 |
| [G-16](#G-16) | [Setting the `constructor` to `payable`](#G-16) | 4 |
| [G-17](#G-17) | [Using `bool` for storage incurs overhead](#G-17) | 4 |
| [G-18](#G-18) | [Use at least Solidity version `0.8.19` to gain some gas boost](#G-18) | 4 |
| [G-19](#G-19) | [Optimize Boolean States with `uint256(1/2)`](#G-19) | 4 |
| [G-20](#G-20) | [Optimize Unsigned Integer Comparison With Zero](#G-20) | 4 |
| [G-21](#G-21) | [Using `private` for constants saves gas](#G-21) | 3 |
| [G-22](#G-22) | [Using `private` for `constants` saves gas](#G-22) | 3 |
| [G-23](#G-23) | [Delete Unused State Variables](#G-23) | 3 |
| [G-24](#G-24) | [Optimize Gas Spend Using `0.8.20` and Optimizer Features](#G-24) | 2 |
| [G-25](#G-25) | [Optimize Gas by Using Only Named Returns](#G-25) | 2 |
| [G-26](#G-26) | [`++i`/`i++` should be `unchecked{++i}`/`unchecked{i++}` when it is not possible for them to overflow, as is the case when used in `for`- and `while`-loops](#G-26) | 1 |
| [G-27](#G-27) | [Pre-increments and pre-decrements are cheaper than post-increments and post-decrements](#G-27) | 1 |
| [G-28](#G-28) | [`Internal` functions only called once can be inlined to save gas](#G-28) | 1 |
| [G-29](#G-29) | [State variables access within a loop](#G-29) | 1 |
| [G-30](#G-30) | [Consider activating `via-ir` for deploying](#G-30) | 1 |
| [G-31](#G-31) | [Optimize Gas by Using Do-While Loops](#G-31) | 1 |

207 instances over 31 issues

---

## Disputed Risk Issues

|Index|Issue|Instances|
|:---:|:---|:---:|
| [D-01](#D-01) | [Control structures do not comply with best practices](#D-01) | 6 |
| [D-02](#D-02) | [Optimize Gas Spend Using `0.8.20` and Optimizer Features](#D-02) | 2 |

8 instances over 2 issues

---



## Medium Risk Issues

### [M-01] Use `_safeMint` instead of `_mint` for `ERC721`
<a name="M-01"></a>
[To the top](#TOP)

`_mint()` is [discouraged](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/d4d8d2ed9798cc3383912a23b5e8d5cb602f7d4b/contracts/token/ERC721/ERC721.sol#L271) in favor of `_safeMint()` which ensures that the recipient is either an EOA or implements IERC721Receiver. Both OpenZeppelin and solmate have versions of this function.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
214	        _mint(msg.sender, _id, _amount, "");
```
[214](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L214)

---

	 - asD/src/asD.sol

```solidity
55	        _mint(msg.sender, _amount);
```
[55](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L55)


</details>


## Low Risk Issues

### [L-01] Functions calling contracts/addresses with transfer hooks are missing reentrancy guards
<a name="L-01"></a>
[To the top](#TOP)

Even if the function follows the best practice of check-effects-interaction, not using a reentrancy guard when there may be transfer hooks will open the users of this protocol up to [read-only reentrancies](https://chainsecurity.com/curve-lp-oracle-manipulation-post-mortem/) with no way to protect against it, except by block-listing the whole protocol.`

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 13 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
150	    function buy(uint256 _id, uint256 _amount) external {
...
// @audit Function `buy` doesn't  have the `nonReentrant` modifier
153	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), price + fee);
...
// @audit Function `buy` doesn't  have the `nonReentrant` modifier
166	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
...
174	    function sell(uint256 _id, uint256 _amount) external {
...
// @audit Function `sell` doesn't  have the `nonReentrant` modifier
187	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim + price - fee);
...
203	    function mintNFT(uint256 _id, uint256 _amount) external {
...
// @audit Function `mintNFT` doesn't  have the `nonReentrant` modifier
206	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
...
// @audit Function `mintNFT` doesn't  have the `nonReentrant` modifier
217	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
...
226	    function burnNFT(uint256 _id, uint256 _amount) external {
...
// @audit Function `burnNFT` doesn't  have the `nonReentrant` modifier
229	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
...
// @audit Function `burnNFT` doesn't  have the `nonReentrant` modifier
238	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
...
244	    function claimPlatformFee() external onlyOwner {
...
// @audit Function `claimPlatformFee` doesn't  have the `nonReentrant` modifier
247	        SafeERC20.safeTransfer(token, msg.sender, amount);
...
253	    function claimCreatorFee(uint256 _id) external {
...
// @audit Function `claimCreatorFee` doesn't  have the `nonReentrant` modifier
257	        SafeERC20.safeTransfer(token, msg.sender, amount);
...
263	    function claimHolderFee(uint256 _id) external {
...
// @audit Function `claimHolderFee` doesn't  have the `nonReentrant` modifier
267	            SafeERC20.safeTransfer(token, msg.sender, amount);
```
[153](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L153)
[166](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L166)
[187](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L187)
[206](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L206)
[217](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L217)
[229](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L229)
[238](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L238)
[247](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L247)
[257](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L257)
[267](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L267)

---

	 - asD/src/asD.sol

```solidity
47	    function mint(uint256 _amount) external {
...
// @audit Function `mint` doesn't  have the `nonReentrant` modifier
50	        SafeERC20.safeTransferFrom(note, msg.sender, address(this), _amount);
...
60	    function burn(uint256 _amount) external {
...
// @audit Function `burn` doesn't  have the `nonReentrant` modifier
66	        SafeERC20.safeTransfer(note, msg.sender, _amount);
...
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
...
// @audit Function `withdrawCarry` doesn't  have the `nonReentrant` modifier
88	        SafeERC20.safeTransfer(note, msg.sender, _amount);
```
[50](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L50)
[66](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L66)
[88](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L88)


</details>

-------
### [L-02] Loss of precision
<a name="L-02"></a>
[To the top](#TOP)

Division by large numbers may result in the result being zero, due to solidity not supporting fractions. Consider requiring a minimum amount for the numerator to ensure that it is always larger than the denominator

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 8 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
...
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
276	            1e18;
...
285	        uint256 shareHolderFee = (_fee * HOLDER_CUT_BPS) / 10_000;
...
286	        uint256 shareCreatorFee = (_fee * CREATOR_CUT_BPS) / 10_000;
...
290	            shareData[_id].shareHolderRewardsPerTokenScaled += (shareHolderFee * 1e18) / _tokenCount;
```
[197](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L197)
[275..276](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L275-L276)
[285](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L285)
[286](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L286)
[290](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L290)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
23	            fee += (getFee(i) * tokenPrice) / 1e18;
...
35	        return 1e17 / divisor;
```
[23](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L23)
[35](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L35)

---

	 - asD/src/asD.sol

```solidity
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
```
[75..76](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L75-L76)


</details>

-------
### [L-03] Privileged functions can create points of failure
<a name="L-03"></a>
[To the top](#TOP)

Ensure such accounts are protected and consider implementing multi sig to prevent a single point of failure

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 7 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
104	    function changeBondingCurveAllowed(address _bondingCurve, bool _newState) external onlyOwner {
...
150	    function buy(uint256 _id, uint256 _amount) external {
...
244	    function claimPlatformFee() external onlyOwner {
...
253	    function claimCreatorFee(uint256 _id) external {
...
300	    function restrictShareCreation(bool _isRestricted) external onlyOwner {
...
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
```
[104](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L104)
[150](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L150)
[244](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L244)
[253](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L253)
[300](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L300)
[309](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L309)

---

	 - asD/src/asD.sol

```solidity
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
```
[72](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L72)


</details>

-------
### [L-04] `onlyOwner` functions not accessible if owner renounces ownership
<a name="L-04"></a>
[To the top](#TOP)

The `owner` is able to perform certain privileged activities, but it's possible to set the owner to `address(0)`. This can represent a certain risk if the ownership is renounced for any other reason than by design.
Renouncing ownership will leave the contract without an `owner`, therefore limiting any functionality that needs authority.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 5 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
104	    function changeBondingCurveAllowed(address _bondingCurve, bool _newState) external onlyOwner {
105	        require(whitelistedBondingCurves[_bondingCurve] != _newState, "State already set");
106	        whitelistedBondingCurves[_bondingCurve] = _newState;
107	        emit BondingCurveStateChange(_bondingCurve, _newState);
108	    }
...
244	    function claimPlatformFee() external onlyOwner {
245	        uint256 amount = platformPool;
246	        platformPool = 0;
247	        SafeERC20.safeTransfer(token, msg.sender, amount);
248	        emit PlatformFeeClaimed(msg.sender, amount);
249	    }
...
300	    function restrictShareCreation(bool _isRestricted) external onlyOwner {
301	        require(shareCreationRestricted != _isRestricted, "State already set");
302	        shareCreationRestricted = _isRestricted;
303	        emit ShareCreationRestricted(_isRestricted);
304	    }
...
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
310	        require(whitelistedShareCreators[_address] != _isWhitelisted, "State already set");
311	        whitelistedShareCreators[_address] = _isWhitelisted;
312	    }
```
[104..108](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L104-L108)
[244..249](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L244-L249)
[300..304](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L300-L304)
[309..312](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L309-L312)

---

	 - asD/src/asD.sol

```solidity
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
73	        uint256 exchangeRate = CTokenInterface(cNote).exchangeRateCurrent(); // Scaled by 1 * 10^(18 - 8 + Underlying Token Decimals), i.e. 10^(28) in our case
74	        // The amount of cNOTE the contract has to hold (based on the current exchange rate which is always increasing) such that it is always possible to receive 1 NOTE when burning 1 asD
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
77	            totalSupply();
78	        if (_amount == 0) {
79	            _amount = maximumWithdrawable;
80	        } else {
81	            require(_amount <= maximumWithdrawable, "Too many tokens requested");
82	        }
83	        // Technically, _amount can still be 0 at this point, which would make the following two calls unnecessary.
84	        // But we do not handle this case specifically, as the only consequence is that the owner wastes a bit of gas when there is nothing to withdraw
85	        uint256 returnCode = CErc20Interface(cNote).redeemUnderlying(_amount);
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
87	        IERC20 note = IERC20(CErc20Interface(cNote).underlying());
88	        SafeERC20.safeTransfer(note, msg.sender, _amount);
89	        emit CarryWithdrawal(_amount);
90	    }
```
[72..90](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L72-L90)


</details>

-------
### [L-05] Code does not follow the best practice of check-effects-interaction
<a name="L-05"></a>
[To the top](#TOP)

Code should follow the best-practice of [check-effects-interaction](https://blockchain-academy.hs-mittweida.de/courses/solidity-coding-beginners-to-intermediate/lessons/solidity-11-coding-patterns/topic/checks-effects-interactions/), where state variables are updated before any external calls are made. Doing so prevents a large class of reentrancy bugs.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 5 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
150	    function buy(uint256 _id, uint256 _amount) external {
151	        require(shareData[_id].creator != msg.sender, "Creator cannot buy");
152	        (uint256 price, uint256 fee) = getBuyPrice(_id, _amount); // Reverts for non-existing ID
153	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), price + fee);
154	        // The reward calculation has to use the old rewards value (pre fee-split) to not include the fees of this buy
155	        // The rewardsLastClaimedValue then needs to be updated with the new value such that the user cannot claim fees of this buy
156	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
157	        // Split the fee among holder, creator and platform
158	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
159	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
160	
161	        shareData[_id].tokenCount += _amount;
162	        shareData[_id].tokensInCirculation += _amount;
163	        tokensByAddress[_id][msg.sender] += _amount;
164	
165	        if (rewardsSinceLastClaim > 0) {
166	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
167	        }
168	        emit SharesBought(_id, msg.sender, _amount, price, fee);
169	    }
...
203	    function mintNFT(uint256 _id, uint256 _amount) external {
204	        uint256 fee = getNFTMintingPrice(_id, _amount);
205	
206	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
207	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
208	        // The user also gets the proportional rewards for the minting
209	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
210	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
211	        tokensByAddress[_id][msg.sender] -= _amount;
212	        shareData[_id].tokensInCirculation -= _amount;
213	
214	        _mint(msg.sender, _id, _amount, "");
215	
216	        if (rewardsSinceLastClaim > 0) {
217	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
218	        }
219	        // ERC1155 already logs, but we add this to have the price information
220	        emit NFTsCreated(_id, msg.sender, _amount, fee);
221	    }
...
226	    function burnNFT(uint256 _id, uint256 _amount) external {
227	        uint256 fee = getNFTMintingPrice(_id, _amount);
228	
229	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
230	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
231	        // The user does not get the proportional rewards for the burning (unless they have additional tokens that are not in the NFT)
232	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
233	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
234	        tokensByAddress[_id][msg.sender] += _amount;
235	        shareData[_id].tokensInCirculation += _amount;
236	        _burn(msg.sender, _id, _amount);
237	
238	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
239	        // ERC1155 already logs, but we add this to have the price information
240	        emit NFTsBurned(_id, msg.sender, _amount, fee);
241	    }
```
[150..169](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L150-L169)
[203..221](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L203-L221)
[226..241](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L226-L241)

---

	 - asD/src/asD.sol

```solidity
47	    function mint(uint256 _amount) external {
48	        CErc20Interface cNoteToken = CErc20Interface(cNote);
49	        IERC20 note = IERC20(cNoteToken.underlying());
50	        SafeERC20.safeTransferFrom(note, msg.sender, address(this), _amount);
51	        SafeERC20.safeApprove(note, cNote, _amount);
52	        uint256 returnCode = cNoteToken.mint(_amount);
53	        // Mint returns 0 on success: https://docs.compound.finance/v2/ctokens/#mint
54	        require(returnCode == 0, "Error when minting");
55	        _mint(msg.sender, _amount);
56	    }
...
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
73	        uint256 exchangeRate = CTokenInterface(cNote).exchangeRateCurrent(); // Scaled by 1 * 10^(18 - 8 + Underlying Token Decimals), i.e. 10^(28) in our case
74	        // The amount of cNOTE the contract has to hold (based on the current exchange rate which is always increasing) such that it is always possible to receive 1 NOTE when burning 1 asD
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
77	            totalSupply();
78	        if (_amount == 0) {
79	            _amount = maximumWithdrawable;
80	        } else {
81	            require(_amount <= maximumWithdrawable, "Too many tokens requested");
82	        }
83	        // Technically, _amount can still be 0 at this point, which would make the following two calls unnecessary.
84	        // But we do not handle this case specifically, as the only consequence is that the owner wastes a bit of gas when there is nothing to withdraw
85	        uint256 returnCode = CErc20Interface(cNote).redeemUnderlying(_amount);
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
87	        IERC20 note = IERC20(CErc20Interface(cNote).underlying());
88	        SafeERC20.safeTransfer(note, msg.sender, _amount);
89	        emit CarryWithdrawal(_amount);
90	    }
```
[47..56](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L47-L56)
[72..90](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L72-L90)


</details>

-------
### [L-06] Prevent division by 0
<a name="L-06"></a>
[To the top](#TOP)

On several locations in the code precautions are not being taken for not dividing by 0, this will revert the code. These functions can be called with 0 value in the input, this value is not checked for being bigger than 0, that means in some scenarios this can potentially trigger a division by zero.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
290	            shareData[_id].shareHolderRewardsPerTokenScaled += (shareHolderFee * 1e18) / _tokenCount;
```
[290](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L290)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
35	        return 1e17 / divisor;
```
[35](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L35)


</details>

-------
### [L-07] Use of `tx.origin` is unsafe in almost every context
<a name="L-07"></a>
[To the top](#TOP)

According to [Vitalik Buterin](https://ethereum.stackexchange.com/questions/196/how-do-i-make-my-dapp-serenity-proof), contracts should _not_ `assume that tx.origin will continue to be usable or meaningful`. An example of this is [EIP-3074](https://eips.ethereum.org/EIPS/eip-3074#allowing-txorigin-as-signer-1) which explicitly mentions the intention to change its semantics when it's used with new op codes. There have also been calls to [remove](https://github.com/ethereum/solidity/issues/683) `tx.origin`, and there are [security issues](solidity.readthedocs.io/en/v0.4.24/security-considerations.html#tx-origin) associated with using it for authorization. For these reasons, it's best to completely avoid the feature.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
96	            turnstile.register(tx.origin);
```
[96](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L96)

---

	 - asD/src/asDFactory.sol

```solidity
29	            turnstile.register(tx.origin);
```
[29](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L29)


</details>

-------
### [L-08] Large assembly blocks should have extensive comments
<a name="L-08"></a>
[To the top](#TOP)

Assembly blocks are take a lot more time to audit than normal Solidity code, and often have gotchas and side-effects that the Solidity versions of the same code do not. Consider adding more comments explaining what is being done in every step of the assembly code

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
44	        assembly {
45	            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
46	            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
47	            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
48	            r := or(r, shl(4, lt(0xffff, shr(r, x))))
49	            r := or(r, shl(3, lt(0xff, shr(r, x))))
50	            // forgefmt: disable-next-item
51	            r := or(
52	                r,
53	                byte(
54	                    and(0x1f, shr(shr(r, x), 0x8421084210842108cc6318c6db6d54be)),
55	                    0x0706060506020504060203020504030106050205030304010505030400000000
56	                )
57	            )
58	        }
```
[44..58](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L44-L58)


</details>

-------
### [L-09] Missing checks for `address(0)` when assigning values to address state variables
<a name="L-09"></a>
[To the top](#TOP)

---

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
114	    function createNewShare(
115	        string memory _shareName,
116	        address _bondingCurve,
117	        string memory _metadataURI
118	    ) external onlyShareCreator returns (uint256 id) {
119	        require(whitelistedBondingCurves[_bondingCurve], "Bonding curve not whitelisted");
120	        require(shareIDs[_shareName] == 0, "Share already exists");
121	        id = ++shareCount;
122	        shareIDs[_shareName] = id;
123	        shareData[id].bondingCurve = _bondingCurve;
124	        shareData[id].creator = msg.sender;
125	        shareData[id].metadataURI = _metadataURI;
126	        emit ShareCreated(id, _shareName, _bondingCurve, msg.sender);
127	    }
```
[114..127](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L114-L127)


</details>


## NonCritical Risk Issues

### [N-01] Incomplete NatSpec `@return` from function declaration
<a name="N-01"></a>
[To the top](#TOP)

Some functions have an incomplete NatSpec: add a `@return` notation to describe the function return value to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 27 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
88	    /// @notice Initiates CSR on main- and testnet
89	    /// @param _uri ERC1155 Base URI
90	    /// @param _paymentToken Address of the payment token
91	    constructor(string memory _uri, address _paymentToken) ERC1155(_uri) Ownable() {
...
100	    /// @notice Whitelist or remove whitelist for a bonding curve.
101	    /// @dev Whitelisting status is only checked when adding a share
102	    /// @param _bondingCurve Address of the bonding curve
103	    /// @param _newState True if whitelisted, false if not
104	    function changeBondingCurveAllowed(address _bondingCurve, bool _newState) external onlyOwner {
...
110	    /// @notice Creates a new share
111	    /// @param _shareName Name of the share
112	    /// @param _bondingCurve Address of the bonding curve, has to be whitelisted
113	    /// @param _metadataURI URI of the metadata
114	    function createNewShare(
115	        string memory _shareName,
116	        address _bondingCurve,
117	        string memory _metadataURI
118	    ) external onlyShareCreator returns (uint256 id) {
...
129	    /// @notice Returns the price and fee for buying a given number of shares.
130	    /// @param _id The ID of the share
131	    /// @param _amount The number of shares to buy.
132	    function getBuyPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
...
138	    /// @notice Returns the price and fee for selling a given number of shares.
139	    /// @param _id The ID of the share
140	    /// @param _amount The number of shares to sell.
141	    function getSellPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
...
147	    /// @notice Buy amount of tokens for a given share ID
148	    /// @param _id ID of the share
149	    /// @param _amount Amount of shares to buy
150	    function buy(uint256 _id, uint256 _amount) external {
...
171	    /// @notice Sell amount of tokens for a given share ID
172	    /// @param _id ID of the share
173	    /// @param _amount Amount of shares to sell
174	    function sell(uint256 _id, uint256 _amount) external {
...
191	    /// @notice Returns the price and fee for minting a given number of NFTs.
192	    /// @param _id The ID of the share
193	    /// @param _amount The number of NFTs to mint.
194	    function getNFTMintingPrice(uint256 _id, uint256 _amount) public view returns (uint256 fee) {
...
200	    /// @notice Convert amount of tokens to NFTs for a given share ID
201	    /// @param _id ID of the share
202	    /// @param _amount Amount of tokens to convert. User needs to have this many tokens.
203	    function mintNFT(uint256 _id, uint256 _amount) external {
...
223	    /// @notice Burn amount of NFTs for a given share ID to get back tokens
224	    /// @param _id ID of the share
225	    /// @param _amount Amount of NFTs to burn
226	    function burnNFT(uint256 _id, uint256 _amount) external {
...
243	    /// @notice Withdraws the accrued platform fee
244	    function claimPlatformFee() external onlyOwner {
...
251	    /// @notice Withdraws the accrued share creator fee
252	    /// @param _id ID of the share
253	    function claimCreatorFee(uint256 _id) external {
...
261	    /// @notice Withdraws the accrued share holder fee
262	    /// @param _id ID of the share
263	    function claimHolderFee(uint256 _id) external {
...
272	    function _getRewardsSinceLastClaim(uint256 _id) internal view returns (uint256 amount) {
...
279	    /// @notice Splits the fee among the share holder, creator and platform
280	    function _splitFees(
281	        uint256 _id,
282	        uint256 _fee,
283	        uint256 _tokenCount
284	    ) internal {
...
298	    /// @notice Restricts or unrestricts share creation
299	    /// @param _isRestricted True if restricted, false if not
300	    function restrictShareCreation(bool _isRestricted) external onlyOwner {
...
306	    /// @notice Adds or removes an address from the whitelist of share creators
307	    /// @param _address Address to add or remove
308	    /// @param _isWhitelisted True if whitelisted, false if not
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
```
[88..91](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L88-L91)
[100..104](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L100-L104)
[110..118](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L110-L118)
[129..132](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L129-L132)
[138..141](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L138-L141)
[147..150](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L147-L150)
[171..174](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L171-L174)
[191..194](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L191-L194)
[200..203](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L200-L203)
[223..226](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L223-L226)
[243..244](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L243-L244)
[251..253](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L251-L253)
[261..263](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L261-L263)
[272](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L272)
[279..284](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L279-L284)
[298..300](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L298-L300)
[306..309](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L306-L309)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
10	    constructor(uint256 _priceIncrease) {
...
14	    function getPriceAndFee(uint256 shareCount, uint256 amount)
15	        external
16	        view
17	        override
18	        returns (uint256 price, uint256 fee)
19	    {
...
27	    function getFee(uint256 shareCount) public pure override returns (uint256) {
...
38	    /// @dev Returns the log2 of `x`.
39	    /// Equivalent to computing the index of the most significant bit (MSB) of `x`.
40	    /// Returns 0 if `x` is zero.
41	    /// @notice Copied from Solady: https://github.com/Vectorized/solady/blob/main/src/utils/FixedPointMathLib.sol
42	    function log2(uint256 x) internal pure returns (uint256 r) {
```
[10](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L10)
[14..19](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L14-L19)
[27](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L27)
[38..42](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L38-L42)

---

	 - asD/src/asD.sol

```solidity
22	    /// @notice Initiates CSR on main- and testnet
23	    /// @param _name Name of the token
24	    /// @param _symbol Symbol of the token
25	    /// @param _owner Initial owner of the vault/token
26	    /// @param _cNote Address of the cNOTE token
27	    /// @param _csrRecipient Address that should receive CSR rewards
28	    constructor(
29	        string memory _name,
30	        string memory _symbol,
31	        address _owner,
32	        address _cNote,
33	        address _csrRecipient
34	    ) ERC20(_name, _symbol) {
...
44	    /// @notice Mint amount of asD tokens by providing NOTE. The NOTE:asD exchange rate is always 1:1
45	    /// @param _amount Amount of tokens to mint
46	    /// @dev User needs to approve the asD contract for _amount of NOTE
47	    function mint(uint256 _amount) external {
...
58	    /// @notice Burn amount of asD tokens to get back NOTE. Like when minting, the NOTE:asD exchange rate is always 1:1
59	    /// @param _amount Amount of tokens to burn
60	    function burn(uint256 _amount) external {
...
69	    /// @notice Withdraw the interest that accrued, only callable by the owner.
70	    /// @param _amount Amount of NOTE to withdraw. 0 for withdrawing the maximum possible amount
71	    /// @dev The function checks that the owner does not withdraw too much NOTE, i.e. that a 1:1 NOTE:asD exchange rate can be maintained after the withdrawal
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
```
[22..34](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L22-L34)
[44..47](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L44-L47)
[58..60](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L58-L60)
[69..72](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L69-L72)

---

	 - asD/src/asDFactory.sol

```solidity
22	    /// @notice Initiates CSR on main- and testnet
23	    /// @param _cNote Address of the cNOTE token
24	    constructor(address _cNote) {
...
33	    function create(string memory _name, string memory _symbol) external returns (address) {
```
[22..24](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L22-L24)
[33](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L33)


</details>

-------
### [N-02] `Constants` in comparisons should appear on the left side
<a name="N-02"></a>
[To the top](#TOP)

Doing so will prevent [typo bugs](https://www.moserware.com/2008/01/constants-on-left-are-better-but-this.html)

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 25 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
93	        if (block.chainid == 7700 || block.chainid == 7701) {
...
93	        if (block.chainid == 7700 || block.chainid == 7701) {
...
120	        require(shareIDs[_shareName] == 0, "Share already exists");
...
135	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount + 1, _amount);
...
144	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount - _amount + 1, _amount);
...
165	        if (rewardsSinceLastClaim > 0) {
...
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
...
216	        if (rewardsSinceLastClaim > 0) {
...
266	        if (amount > 0) {
...
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
276	            1e18;
...
285	        uint256 shareHolderFee = (_fee * HOLDER_CUT_BPS) / 10_000;
...
286	        uint256 shareCreatorFee = (_fee * CREATOR_CUT_BPS) / 10_000;
...
289	        if (_tokenCount > 0) {
...
290	            shareData[_id].shareHolderRewardsPerTokenScaled += (shareHolderFee * 1e18) / _tokenCount;
```
[93](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L93)
[93](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L93)
[120](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L120)
[135](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L135)
[144](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L144)
[165](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L165)
[197](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L197)
[216](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L216)
[266](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L266)
[275..276](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L275-L276)
[285](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L285)
[286](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L286)
[289](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L289)
[290](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L290)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
23	            fee += (getFee(i) * tokenPrice) / 1e18;
...
29	        if (shareCount > 1) {
```
[23](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L23)
[29](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L29)

---

	 - asD/src/asD.sol

```solidity
37	        if (block.chainid == 7700 || block.chainid == 7701) {
...
37	        if (block.chainid == 7700 || block.chainid == 7701) {
...
54	        require(returnCode == 0, "Error when minting");
...
64	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem-underlying
...
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
...
78	        if (_amount == 0) {
...
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
```
[37](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L37)
[37](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L37)
[54](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L54)
[64](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L64)
[75..76](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L75-L76)
[78](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L78)
[86](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L86)

---

	 - asD/src/asDFactory.sol

```solidity
26	        if (block.chainid == 7700 || block.chainid == 7701) {
...
26	        if (block.chainid == 7700 || block.chainid == 7701) {
```
[26](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L26)
[26](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L26)


</details>

-------
### [N-03] Missing NatSpec `@dev` from function declaration
<a name="N-03"></a>
[To the top](#TOP)

Some functions have an incomplete NatSpec: add a `@dev` notation to describe the function to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 23 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
88	    /// @notice Initiates CSR on main- and testnet
89	    /// @param _uri ERC1155 Base URI
90	    /// @param _paymentToken Address of the payment token
91	    constructor(string memory _uri, address _paymentToken) ERC1155(_uri) Ownable() {
...
110	    /// @notice Creates a new share
111	    /// @param _shareName Name of the share
112	    /// @param _bondingCurve Address of the bonding curve, has to be whitelisted
113	    /// @param _metadataURI URI of the metadata
114	    function createNewShare(
115	        string memory _shareName,
116	        address _bondingCurve,
117	        string memory _metadataURI
118	    ) external onlyShareCreator returns (uint256 id) {
...
129	    /// @notice Returns the price and fee for buying a given number of shares.
130	    /// @param _id The ID of the share
131	    /// @param _amount The number of shares to buy.
132	    function getBuyPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
...
138	    /// @notice Returns the price and fee for selling a given number of shares.
139	    /// @param _id The ID of the share
140	    /// @param _amount The number of shares to sell.
141	    function getSellPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
...
147	    /// @notice Buy amount of tokens for a given share ID
148	    /// @param _id ID of the share
149	    /// @param _amount Amount of shares to buy
150	    function buy(uint256 _id, uint256 _amount) external {
...
171	    /// @notice Sell amount of tokens for a given share ID
172	    /// @param _id ID of the share
173	    /// @param _amount Amount of shares to sell
174	    function sell(uint256 _id, uint256 _amount) external {
...
191	    /// @notice Returns the price and fee for minting a given number of NFTs.
192	    /// @param _id The ID of the share
193	    /// @param _amount The number of NFTs to mint.
194	    function getNFTMintingPrice(uint256 _id, uint256 _amount) public view returns (uint256 fee) {
...
200	    /// @notice Convert amount of tokens to NFTs for a given share ID
201	    /// @param _id ID of the share
202	    /// @param _amount Amount of tokens to convert. User needs to have this many tokens.
203	    function mintNFT(uint256 _id, uint256 _amount) external {
...
223	    /// @notice Burn amount of NFTs for a given share ID to get back tokens
224	    /// @param _id ID of the share
225	    /// @param _amount Amount of NFTs to burn
226	    function burnNFT(uint256 _id, uint256 _amount) external {
...
243	    /// @notice Withdraws the accrued platform fee
244	    function claimPlatformFee() external onlyOwner {
...
251	    /// @notice Withdraws the accrued share creator fee
252	    /// @param _id ID of the share
253	    function claimCreatorFee(uint256 _id) external {
...
261	    /// @notice Withdraws the accrued share holder fee
262	    /// @param _id ID of the share
263	    function claimHolderFee(uint256 _id) external {
...
272	    function _getRewardsSinceLastClaim(uint256 _id) internal view returns (uint256 amount) {
...
279	    /// @notice Splits the fee among the share holder, creator and platform
280	    function _splitFees(
281	        uint256 _id,
282	        uint256 _fee,
283	        uint256 _tokenCount
284	    ) internal {
...
298	    /// @notice Restricts or unrestricts share creation
299	    /// @param _isRestricted True if restricted, false if not
300	    function restrictShareCreation(bool _isRestricted) external onlyOwner {
...
306	    /// @notice Adds or removes an address from the whitelist of share creators
307	    /// @param _address Address to add or remove
308	    /// @param _isWhitelisted True if whitelisted, false if not
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
```
[88..91](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L88-L91)
[110..118](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L110-L118)
[129..132](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L129-L132)
[138..141](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L138-L141)
[147..150](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L147-L150)
[171..174](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L171-L174)
[191..194](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L191-L194)
[200..203](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L200-L203)
[223..226](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L223-L226)
[243..244](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L243-L244)
[251..253](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L251-L253)
[261..263](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L261-L263)
[272](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L272)
[279..284](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L279-L284)
[298..300](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L298-L300)
[306..309](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L306-L309)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
10	    constructor(uint256 _priceIncrease) {
...
14	    function getPriceAndFee(uint256 shareCount, uint256 amount)
15	        external
16	        view
17	        override
18	        returns (uint256 price, uint256 fee)
19	    {
...
27	    function getFee(uint256 shareCount) public pure override returns (uint256) {
```
[10](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L10)
[14..19](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L14-L19)
[27](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L27)

---

	 - asD/src/asD.sol

```solidity
22	    /// @notice Initiates CSR on main- and testnet
23	    /// @param _name Name of the token
24	    /// @param _symbol Symbol of the token
25	    /// @param _owner Initial owner of the vault/token
26	    /// @param _cNote Address of the cNOTE token
27	    /// @param _csrRecipient Address that should receive CSR rewards
28	    constructor(
29	        string memory _name,
30	        string memory _symbol,
31	        address _owner,
32	        address _cNote,
33	        address _csrRecipient
34	    ) ERC20(_name, _symbol) {
...
58	    /// @notice Burn amount of asD tokens to get back NOTE. Like when minting, the NOTE:asD exchange rate is always 1:1
59	    /// @param _amount Amount of tokens to burn
60	    function burn(uint256 _amount) external {
```
[22..34](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L22-L34)
[58..60](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L58-L60)

---

	 - asD/src/asDFactory.sol

```solidity
22	    /// @notice Initiates CSR on main- and testnet
23	    /// @param _cNote Address of the cNOTE token
24	    constructor(address _cNote) {
...
33	    function create(string memory _name, string memory _symbol) external returns (address) {
```
[22..24](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L22-L24)
[33](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L33)


</details>

-------
### [N-04] `constants` should be defined rather than using magic numbers
<a name="N-04"></a>
[To the top](#TOP)

Magic numbers are numbers that appear without explanation in the code. They should be replaced with named constants.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 22 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
93	        if (block.chainid == 7700 || block.chainid == 7701) {
...
93	        if (block.chainid == 7700 || block.chainid == 7701) {
...
95	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
...
135	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount + 1, _amount);
...
144	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount - _amount + 1, _amount);
...
196	        (uint256 priceForOne, ) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount, 1);
...
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
...
276	            1e18;
...
285	        uint256 shareHolderFee = (_fee * HOLDER_CUT_BPS) / 10_000;
...
286	        uint256 shareCreatorFee = (_fee * CREATOR_CUT_BPS) / 10_000;
...
290	            shareData[_id].shareHolderRewardsPerTokenScaled += (shareHolderFee * 1e18) / _tokenCount;
```
[93](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L93)
[93](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L93)
[95](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L95)
[135](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L135)
[144](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L144)
[196](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L196)
[197](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L197)
[276](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L276)
[285](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L285)
[286](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L286)
[290](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L290)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
23	            fee += (getFee(i) * tokenPrice) / 1e18;
...
29	        if (shareCount > 1) {
...
32	            divisor = 1;
...
35	        return 1e17 / divisor;
```
[23](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L23)
[29](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L29)
[32](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L32)
[35](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L35)

---

	 - asD/src/asD.sol

```solidity
37	        if (block.chainid == 7700 || block.chainid == 7701) {
...
37	        if (block.chainid == 7700 || block.chainid == 7701) {
...
39	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
...
76	            1e28 -
```
[37](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L37)
[37](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L37)
[39](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L39)
[76](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L76)

---

	 - asD/src/asDFactory.sol

```solidity
26	        if (block.chainid == 7700 || block.chainid == 7701) {
...
26	        if (block.chainid == 7700 || block.chainid == 7701) {
...
28	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
```
[26](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L26)
[26](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L26)
[28](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L28)


</details>

-------
### [N-05] Contract functions should use an `interface`
<a name="N-05"></a>
[To the top](#TOP)

All `external`/`public` functions should extend an `interface`. This is useful to make sure that the whole API is extracted.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 18 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
104	    function changeBondingCurveAllowed(address _bondingCurve, bool _newState) external onlyOwner {
105	        require(whitelistedBondingCurves[_bondingCurve] != _newState, "State already set");
106	        whitelistedBondingCurves[_bondingCurve] = _newState;
107	        emit BondingCurveStateChange(_bondingCurve, _newState);
108	    }
...
114	    function createNewShare(
115	        string memory _shareName,
116	        address _bondingCurve,
117	        string memory _metadataURI
118	    ) external onlyShareCreator returns (uint256 id) {
119	        require(whitelistedBondingCurves[_bondingCurve], "Bonding curve not whitelisted");
120	        require(shareIDs[_shareName] == 0, "Share already exists");
121	        id = ++shareCount;
122	        shareIDs[_shareName] = id;
123	        shareData[id].bondingCurve = _bondingCurve;
124	        shareData[id].creator = msg.sender;
125	        shareData[id].metadataURI = _metadataURI;
126	        emit ShareCreated(id, _shareName, _bondingCurve, msg.sender);
127	    }
...
132	    function getBuyPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
133	        // If id does not exist, this will return address(0), causing a revert in the next line
134	        address bondingCurve = shareData[_id].bondingCurve;
135	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount + 1, _amount);
136	    }
...
141	    function getSellPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
142	        // If id does not exist, this will return address(0), causing a revert in the next line
143	        address bondingCurve = shareData[_id].bondingCurve;
144	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount - _amount + 1, _amount);
145	    }
...
150	    function buy(uint256 _id, uint256 _amount) external {
151	        require(shareData[_id].creator != msg.sender, "Creator cannot buy");
152	        (uint256 price, uint256 fee) = getBuyPrice(_id, _amount); // Reverts for non-existing ID
153	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), price + fee);
154	        // The reward calculation has to use the old rewards value (pre fee-split) to not include the fees of this buy
155	        // The rewardsLastClaimedValue then needs to be updated with the new value such that the user cannot claim fees of this buy
156	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
157	        // Split the fee among holder, creator and platform
158	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
159	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
160	
161	        shareData[_id].tokenCount += _amount;
162	        shareData[_id].tokensInCirculation += _amount;
163	        tokensByAddress[_id][msg.sender] += _amount;
164	
165	        if (rewardsSinceLastClaim > 0) {
166	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
167	        }
168	        emit SharesBought(_id, msg.sender, _amount, price, fee);
169	    }
...
174	    function sell(uint256 _id, uint256 _amount) external {
175	        (uint256 price, uint256 fee) = getSellPrice(_id, _amount);
176	        // Split the fee among holder, creator and platform
177	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
178	        // The user also gets the rewards of his own sale (which is not the case for buys)
179	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
180	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
181	
182	        shareData[_id].tokenCount -= _amount;
183	        shareData[_id].tokensInCirculation -= _amount;
184	        tokensByAddress[_id][msg.sender] -= _amount; // Would underflow if user did not have enough tokens
185	
186	        // Send the funds to the user
187	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim + price - fee);
188	        emit SharesSold(_id, msg.sender, _amount, price, fee);
189	    }
...
194	    function getNFTMintingPrice(uint256 _id, uint256 _amount) public view returns (uint256 fee) {
195	        address bondingCurve = shareData[_id].bondingCurve;
196	        (uint256 priceForOne, ) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount, 1);
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
198	    }
...
203	    function mintNFT(uint256 _id, uint256 _amount) external {
204	        uint256 fee = getNFTMintingPrice(_id, _amount);
205	
206	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
207	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
208	        // The user also gets the proportional rewards for the minting
209	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
210	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
211	        tokensByAddress[_id][msg.sender] -= _amount;
212	        shareData[_id].tokensInCirculation -= _amount;
213	
214	        _mint(msg.sender, _id, _amount, "");
215	
216	        if (rewardsSinceLastClaim > 0) {
217	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
218	        }
219	        // ERC1155 already logs, but we add this to have the price information
220	        emit NFTsCreated(_id, msg.sender, _amount, fee);
221	    }
...
226	    function burnNFT(uint256 _id, uint256 _amount) external {
227	        uint256 fee = getNFTMintingPrice(_id, _amount);
228	
229	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
230	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
231	        // The user does not get the proportional rewards for the burning (unless they have additional tokens that are not in the NFT)
232	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
233	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
234	        tokensByAddress[_id][msg.sender] += _amount;
235	        shareData[_id].tokensInCirculation += _amount;
236	        _burn(msg.sender, _id, _amount);
237	
238	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
239	        // ERC1155 already logs, but we add this to have the price information
240	        emit NFTsBurned(_id, msg.sender, _amount, fee);
241	    }
...
244	    function claimPlatformFee() external onlyOwner {
245	        uint256 amount = platformPool;
246	        platformPool = 0;
247	        SafeERC20.safeTransfer(token, msg.sender, amount);
248	        emit PlatformFeeClaimed(msg.sender, amount);
249	    }
...
253	    function claimCreatorFee(uint256 _id) external {
254	        require(shareData[_id].creator == msg.sender, "Not creator");
255	        uint256 amount = shareData[_id].shareCreatorPool;
256	        shareData[_id].shareCreatorPool = 0;
257	        SafeERC20.safeTransfer(token, msg.sender, amount);
258	        emit CreatorFeeClaimed(msg.sender, _id, amount);
259	    }
...
263	    function claimHolderFee(uint256 _id) external {
264	        uint256 amount = _getRewardsSinceLastClaim(_id);
265	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
266	        if (amount > 0) {
267	            SafeERC20.safeTransfer(token, msg.sender, amount);
268	        }
269	        emit HolderFeeClaimed(msg.sender, _id, amount);
270	    }
...
300	    function restrictShareCreation(bool _isRestricted) external onlyOwner {
301	        require(shareCreationRestricted != _isRestricted, "State already set");
302	        shareCreationRestricted = _isRestricted;
303	        emit ShareCreationRestricted(_isRestricted);
304	    }
...
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
310	        require(whitelistedShareCreators[_address] != _isWhitelisted, "State already set");
311	        whitelistedShareCreators[_address] = _isWhitelisted;
312	    }
```
[104..108](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L104-L108)
[114..127](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L114-L127)
[132..136](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L132-L136)
[141..145](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L141-L145)
[150..169](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L150-L169)
[174..189](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L174-L189)
[194..198](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L194-L198)
[203..221](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L203-L221)
[226..241](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L226-L241)
[244..249](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L244-L249)
[253..259](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L253-L259)
[263..270](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L263-L270)
[300..304](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L300-L304)
[309..312](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L309-L312)

---

	 - asD/src/asD.sol

```solidity
47	    function mint(uint256 _amount) external {
48	        CErc20Interface cNoteToken = CErc20Interface(cNote);
49	        IERC20 note = IERC20(cNoteToken.underlying());
50	        SafeERC20.safeTransferFrom(note, msg.sender, address(this), _amount);
51	        SafeERC20.safeApprove(note, cNote, _amount);
52	        uint256 returnCode = cNoteToken.mint(_amount);
53	        // Mint returns 0 on success: https://docs.compound.finance/v2/ctokens/#mint
54	        require(returnCode == 0, "Error when minting");
55	        _mint(msg.sender, _amount);
56	    }
...
60	    function burn(uint256 _amount) external {
61	        CErc20Interface cNoteToken = CErc20Interface(cNote);
62	        IERC20 note = IERC20(cNoteToken.underlying());
63	        uint256 returnCode = cNoteToken.redeemUnderlying(_amount); // Request _amount of NOTE (the underlying of cNOTE)
64	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem-underlying
65	        _burn(msg.sender, _amount);
66	        SafeERC20.safeTransfer(note, msg.sender, _amount);
67	    }
...
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
73	        uint256 exchangeRate = CTokenInterface(cNote).exchangeRateCurrent(); // Scaled by 1 * 10^(18 - 8 + Underlying Token Decimals), i.e. 10^(28) in our case
74	        // The amount of cNOTE the contract has to hold (based on the current exchange rate which is always increasing) such that it is always possible to receive 1 NOTE when burning 1 asD
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
77	            totalSupply();
78	        if (_amount == 0) {
79	            _amount = maximumWithdrawable;
80	        } else {
81	            require(_amount <= maximumWithdrawable, "Too many tokens requested");
82	        }
83	        // Technically, _amount can still be 0 at this point, which would make the following two calls unnecessary.
84	        // But we do not handle this case specifically, as the only consequence is that the owner wastes a bit of gas when there is nothing to withdraw
85	        uint256 returnCode = CErc20Interface(cNote).redeemUnderlying(_amount);
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
87	        IERC20 note = IERC20(CErc20Interface(cNote).underlying());
88	        SafeERC20.safeTransfer(note, msg.sender, _amount);
89	        emit CarryWithdrawal(_amount);
90	    }
```
[47..56](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L47-L56)
[60..67](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L60-L67)
[72..90](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L72-L90)

---

	 - asD/src/asDFactory.sol

```solidity
33	    function create(string memory _name, string memory _symbol) external returns (address) {
34	        asD createdToken = new asD(_name, _symbol, msg.sender, cNote, owner());
35	        isAsD[address(createdToken)] = true;
36	        emit CreatedToken(address(createdToken), _symbol, _name, msg.sender);
37	        return address(createdToken);
38	    }
```
[33..38](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L33-L38)


</details>

-------
### [N-06] Custom `error` should be used rather than `require`/`assert`
<a name="N-06"></a>
[To the top](#TOP)

Custom errors are available from solidity version 0.8.4. Custom errors are more easily processed in try-catch blocks, and are easier to re-use and maintain.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 12 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
81	        require(
82	            !shareCreationRestricted || whitelistedShareCreators[msg.sender] || msg.sender == owner(),
83	            "Not allowed"
84	        );
...
105	        require(whitelistedBondingCurves[_bondingCurve] != _newState, "State already set");
...
119	        require(whitelistedBondingCurves[_bondingCurve], "Bonding curve not whitelisted");
...
120	        require(shareIDs[_shareName] == 0, "Share already exists");
...
151	        require(shareData[_id].creator != msg.sender, "Creator cannot buy");
...
254	        require(shareData[_id].creator == msg.sender, "Not creator");
...
301	        require(shareCreationRestricted != _isRestricted, "State already set");
...
310	        require(whitelistedShareCreators[_address] != _isWhitelisted, "State already set");
```
[81..84](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L81-L84)
[105](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L105)
[119](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L119)
[120](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L120)
[151](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L151)
[254](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L254)
[301](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L301)
[310](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L310)

---

	 - asD/src/asD.sol

```solidity
54	        require(returnCode == 0, "Error when minting");
...
64	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem-underlying
...
81	            require(_amount <= maximumWithdrawable, "Too many tokens requested");
...
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
```
[54](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L54)
[64](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L64)
[81](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L81)
[86](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L86)


</details>

-------
### [N-07] Event declarations should have NatSpec descriptions
<a name="N-07"></a>
[To the top](#TOP)

---

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 12 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
69	    event BondingCurveStateChange(address indexed curve, bool isWhitelisted);
...
70	    event ShareCreated(uint256 indexed id, string name, address indexed bondingCurve, address indexed creator);
...
71	    event SharesBought(uint256 indexed id, address indexed buyer, uint256 amount, uint256 price, uint256 fee);
...
72	    event SharesSold(uint256 indexed id, address indexed seller, uint256 amount, uint256 price, uint256 fee);
...
73	    event NFTsCreated(uint256 indexed id, address indexed creator, uint256 amount, uint256 fee);
...
74	    event NFTsBurned(uint256 indexed id, address indexed burner, uint256 amount, uint256 fee);
...
75	    event PlatformFeeClaimed(address indexed claimer, uint256 amount);
...
76	    event CreatorFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
77	    event HolderFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
78	    event ShareCreationRestricted(bool isRestricted);
```
[69](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L69)
[70](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L70)
[71](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L71)
[72](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L72)
[73](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L73)
[74](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L74)
[75](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L75)
[76](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L76)
[77](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L77)
[78](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L78)

---

	 - asD/src/asD.sol

```solidity
20	    event CarryWithdrawal(uint256 amount);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L20)

---

	 - asD/src/asDFactory.sol

```solidity
20	    event CreatedToken(address token, string symbol, string name, address creator);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L20)


</details>

-------
### [N-08] Missing NatSpec from event declaration
<a name="N-08"></a>
[To the top](#TOP)

Consider adding some comments on event declarations to explain what they are supposed to do: this will help for future code reviews.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 12 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
69	    event BondingCurveStateChange(address indexed curve, bool isWhitelisted);
...
70	    event ShareCreated(uint256 indexed id, string name, address indexed bondingCurve, address indexed creator);
...
71	    event SharesBought(uint256 indexed id, address indexed buyer, uint256 amount, uint256 price, uint256 fee);
...
72	    event SharesSold(uint256 indexed id, address indexed seller, uint256 amount, uint256 price, uint256 fee);
...
73	    event NFTsCreated(uint256 indexed id, address indexed creator, uint256 amount, uint256 fee);
...
74	    event NFTsBurned(uint256 indexed id, address indexed burner, uint256 amount, uint256 fee);
...
75	    event PlatformFeeClaimed(address indexed claimer, uint256 amount);
...
76	    event CreatorFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
77	    event HolderFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
78	    event ShareCreationRestricted(bool isRestricted);
```
[69](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L69)
[70](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L70)
[71](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L71)
[72](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L72)
[73](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L73)
[74](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L74)
[75](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L75)
[76](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L76)
[77](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L77)
[78](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L78)

---

	 - asD/src/asD.sol

```solidity
20	    event CarryWithdrawal(uint256 amount);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L20)

---

	 - asD/src/asDFactory.sol

```solidity
20	    event CreatedToken(address token, string symbol, string name, address creator);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L20)


</details>

-------
### [N-09] Missing NatSpec `@dev` from event declaration
<a name="N-09"></a>
[To the top](#TOP)

Some events have an incomplete NatSpec: add a `@dev` notation to describe the event to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 12 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
69	    event BondingCurveStateChange(address indexed curve, bool isWhitelisted);
...
70	    event ShareCreated(uint256 indexed id, string name, address indexed bondingCurve, address indexed creator);
...
71	    event SharesBought(uint256 indexed id, address indexed buyer, uint256 amount, uint256 price, uint256 fee);
...
72	    event SharesSold(uint256 indexed id, address indexed seller, uint256 amount, uint256 price, uint256 fee);
...
73	    event NFTsCreated(uint256 indexed id, address indexed creator, uint256 amount, uint256 fee);
...
74	    event NFTsBurned(uint256 indexed id, address indexed burner, uint256 amount, uint256 fee);
...
75	    event PlatformFeeClaimed(address indexed claimer, uint256 amount);
...
76	    event CreatorFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
77	    event HolderFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
78	    event ShareCreationRestricted(bool isRestricted);
```
[69](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L69)
[70](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L70)
[71](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L71)
[72](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L72)
[73](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L73)
[74](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L74)
[75](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L75)
[76](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L76)
[77](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L77)
[78](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L78)

---

	 - asD/src/asD.sol

```solidity
20	    event CarryWithdrawal(uint256 amount);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L20)

---

	 - asD/src/asDFactory.sol

```solidity
20	    event CreatedToken(address token, string symbol, string name, address creator);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L20)


</details>

-------
### [N-10] Missing NatSpec `@notice` from event declaration
<a name="N-10"></a>
[To the top](#TOP)

Some events have an incomplete NatSpec: add a `@notice` notation to describe the event to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 12 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
69	    event BondingCurveStateChange(address indexed curve, bool isWhitelisted);
...
70	    event ShareCreated(uint256 indexed id, string name, address indexed bondingCurve, address indexed creator);
...
71	    event SharesBought(uint256 indexed id, address indexed buyer, uint256 amount, uint256 price, uint256 fee);
...
72	    event SharesSold(uint256 indexed id, address indexed seller, uint256 amount, uint256 price, uint256 fee);
...
73	    event NFTsCreated(uint256 indexed id, address indexed creator, uint256 amount, uint256 fee);
...
74	    event NFTsBurned(uint256 indexed id, address indexed burner, uint256 amount, uint256 fee);
...
75	    event PlatformFeeClaimed(address indexed claimer, uint256 amount);
...
76	    event CreatorFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
77	    event HolderFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
78	    event ShareCreationRestricted(bool isRestricted);
```
[69](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L69)
[70](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L70)
[71](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L71)
[72](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L72)
[73](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L73)
[74](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L74)
[75](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L75)
[76](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L76)
[77](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L77)
[78](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L78)

---

	 - asD/src/asD.sol

```solidity
20	    event CarryWithdrawal(uint256 amount);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L20)

---

	 - asD/src/asDFactory.sol

```solidity
20	    event CreatedToken(address token, string symbol, string name, address creator);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L20)


</details>

-------
### [N-11] Missing NatSpec `@param` from event declaration
<a name="N-11"></a>
[To the top](#TOP)

Some events have an incomplete NatSpec: add a `@param` notation to describe the event to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 12 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
69	    event BondingCurveStateChange(address indexed curve, bool isWhitelisted);
...
70	    event ShareCreated(uint256 indexed id, string name, address indexed bondingCurve, address indexed creator);
...
71	    event SharesBought(uint256 indexed id, address indexed buyer, uint256 amount, uint256 price, uint256 fee);
...
72	    event SharesSold(uint256 indexed id, address indexed seller, uint256 amount, uint256 price, uint256 fee);
...
73	    event NFTsCreated(uint256 indexed id, address indexed creator, uint256 amount, uint256 fee);
...
74	    event NFTsBurned(uint256 indexed id, address indexed burner, uint256 amount, uint256 fee);
...
75	    event PlatformFeeClaimed(address indexed claimer, uint256 amount);
...
76	    event CreatorFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
77	    event HolderFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
78	    event ShareCreationRestricted(bool isRestricted);
```
[69](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L69)
[70](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L70)
[71](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L71)
[72](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L72)
[73](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L73)
[74](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L74)
[75](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L75)
[76](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L76)
[77](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L77)
[78](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L78)

---

	 - asD/src/asD.sol

```solidity
20	    event CarryWithdrawal(uint256 amount);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L20)

---

	 - asD/src/asDFactory.sol

```solidity
20	    event CreatedToken(address token, string symbol, string name, address creator);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L20)


</details>

-------
### [N-12] Event is missing `indexed` field
<a name="N-12"></a>
[To the top](#TOP)

---

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 11 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
69	    event BondingCurveStateChange(address indexed curve, bool isWhitelisted);
...
71	    event SharesBought(uint256 indexed id, address indexed buyer, uint256 amount, uint256 price, uint256 fee);
...
72	    event SharesSold(uint256 indexed id, address indexed seller, uint256 amount, uint256 price, uint256 fee);
...
73	    event NFTsCreated(uint256 indexed id, address indexed creator, uint256 amount, uint256 fee);
...
74	    event NFTsBurned(uint256 indexed id, address indexed burner, uint256 amount, uint256 fee);
...
75	    event PlatformFeeClaimed(address indexed claimer, uint256 amount);
...
76	    event CreatorFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
77	    event HolderFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
...
78	    event ShareCreationRestricted(bool isRestricted);
```
[69](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L69)
[71](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L71)
[72](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L72)
[73](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L73)
[74](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L74)
[75](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L75)
[76](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L76)
[77](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L77)
[78](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L78)

---

	 - asD/src/asD.sol

```solidity
20	    event CarryWithdrawal(uint256 amount);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L20)

---

	 - asD/src/asDFactory.sol

```solidity
20	    event CreatedToken(address token, string symbol, string name, address creator);
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L20)


</details>

-------
### [N-13] Using named parameters in `mapping` is best practice
<a name="N-13"></a>
[To the top](#TOP)

---

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 8 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
30	    mapping(string => uint256) public shareIDs;
...
43	    mapping(uint256 => ShareData) public shareData;
...
46	    mapping(uint256 => address) public shareBondingCurves;
...
49	    mapping(address => bool) public whitelistedBondingCurves;
...
52	    mapping(uint256 => mapping(address => uint256)) public tokensByAddress;
...
55	    mapping(uint256 => mapping(address => uint256)) public rewardsLastClaimedValue;
...
64	    mapping(address => bool) public whitelistedShareCreators;
```
[30](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L30)
[43](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L43)
[46](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L46)
[49](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L49)
[52](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L52)
[55](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L55)
[64](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L64)

---

	 - asD/src/asDFactory.sol

```solidity
15	    mapping(address => bool) public isAsD;
```
[15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L15)


</details>

-------
### [N-14] Events may be emitted out of order due to reentrancy
<a name="N-14"></a>
[To the top](#TOP)

Ensure that events follow the best practice of check-effects-interaction, and are emitted before external calls

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 8 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
150	    function buy(uint256 _id, uint256 _amount) external {
151	        require(shareData[_id].creator != msg.sender, "Creator cannot buy");
152	        (uint256 price, uint256 fee) = getBuyPrice(_id, _amount); // Reverts for non-existing ID
153	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), price + fee);
154	        // The reward calculation has to use the old rewards value (pre fee-split) to not include the fees of this buy
155	        // The rewardsLastClaimedValue then needs to be updated with the new value such that the user cannot claim fees of this buy
156	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
157	        // Split the fee among holder, creator and platform
158	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
159	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
160	
161	        shareData[_id].tokenCount += _amount;
162	        shareData[_id].tokensInCirculation += _amount;
163	        tokensByAddress[_id][msg.sender] += _amount;
164	
165	        if (rewardsSinceLastClaim > 0) {
166	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
167	        }
168	        emit SharesBought(_id, msg.sender, _amount, price, fee);
169	    }
...
174	    function sell(uint256 _id, uint256 _amount) external {
175	        (uint256 price, uint256 fee) = getSellPrice(_id, _amount);
176	        // Split the fee among holder, creator and platform
177	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
178	        // The user also gets the rewards of his own sale (which is not the case for buys)
179	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
180	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
181	
182	        shareData[_id].tokenCount -= _amount;
183	        shareData[_id].tokensInCirculation -= _amount;
184	        tokensByAddress[_id][msg.sender] -= _amount; // Would underflow if user did not have enough tokens
185	
186	        // Send the funds to the user
187	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim + price - fee);
188	        emit SharesSold(_id, msg.sender, _amount, price, fee);
189	    }
...
203	    function mintNFT(uint256 _id, uint256 _amount) external {
204	        uint256 fee = getNFTMintingPrice(_id, _amount);
205	
206	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
207	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
208	        // The user also gets the proportional rewards for the minting
209	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
210	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
211	        tokensByAddress[_id][msg.sender] -= _amount;
212	        shareData[_id].tokensInCirculation -= _amount;
213	
214	        _mint(msg.sender, _id, _amount, "");
215	
216	        if (rewardsSinceLastClaim > 0) {
217	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
218	        }
219	        // ERC1155 already logs, but we add this to have the price information
220	        emit NFTsCreated(_id, msg.sender, _amount, fee);
221	    }
...
226	    function burnNFT(uint256 _id, uint256 _amount) external {
227	        uint256 fee = getNFTMintingPrice(_id, _amount);
228	
229	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
230	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
231	        // The user does not get the proportional rewards for the burning (unless they have additional tokens that are not in the NFT)
232	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
233	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
234	        tokensByAddress[_id][msg.sender] += _amount;
235	        shareData[_id].tokensInCirculation += _amount;
236	        _burn(msg.sender, _id, _amount);
237	
238	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
239	        // ERC1155 already logs, but we add this to have the price information
240	        emit NFTsBurned(_id, msg.sender, _amount, fee);
241	    }
...
244	    function claimPlatformFee() external onlyOwner {
245	        uint256 amount = platformPool;
246	        platformPool = 0;
247	        SafeERC20.safeTransfer(token, msg.sender, amount);
248	        emit PlatformFeeClaimed(msg.sender, amount);
249	    }
...
253	    function claimCreatorFee(uint256 _id) external {
254	        require(shareData[_id].creator == msg.sender, "Not creator");
255	        uint256 amount = shareData[_id].shareCreatorPool;
256	        shareData[_id].shareCreatorPool = 0;
257	        SafeERC20.safeTransfer(token, msg.sender, amount);
258	        emit CreatorFeeClaimed(msg.sender, _id, amount);
259	    }
...
263	    function claimHolderFee(uint256 _id) external {
264	        uint256 amount = _getRewardsSinceLastClaim(_id);
265	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
266	        if (amount > 0) {
267	            SafeERC20.safeTransfer(token, msg.sender, amount);
268	        }
269	        emit HolderFeeClaimed(msg.sender, _id, amount);
270	    }
```
[150..169](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L150-L169)
[174..189](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L174-L189)
[203..221](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L203-L221)
[226..241](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L226-L241)
[244..249](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L244-L249)
[253..259](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L253-L259)
[263..270](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L263-L270)

---

	 - asD/src/asD.sol

```solidity
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
73	        uint256 exchangeRate = CTokenInterface(cNote).exchangeRateCurrent(); // Scaled by 1 * 10^(18 - 8 + Underlying Token Decimals), i.e. 10^(28) in our case
74	        // The amount of cNOTE the contract has to hold (based on the current exchange rate which is always increasing) such that it is always possible to receive 1 NOTE when burning 1 asD
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
77	            totalSupply();
78	        if (_amount == 0) {
79	            _amount = maximumWithdrawable;
80	        } else {
81	            require(_amount <= maximumWithdrawable, "Too many tokens requested");
82	        }
83	        // Technically, _amount can still be 0 at this point, which would make the following two calls unnecessary.
84	        // But we do not handle this case specifically, as the only consequence is that the owner wastes a bit of gas when there is nothing to withdraw
85	        uint256 returnCode = CErc20Interface(cNote).redeemUnderlying(_amount);
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
87	        IERC20 note = IERC20(CErc20Interface(cNote).underlying());
88	        SafeERC20.safeTransfer(note, msg.sender, _amount);
89	        emit CarryWithdrawal(_amount);
90	    }
```
[72..90](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L72-L90)


</details>

-------
### [N-15] Missing NatSpec `@param` from function declaration
<a name="N-15"></a>
[To the top](#TOP)

Some functions have an incomplete NatSpec: add a `@param` notation to describe the function parameters to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 8 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
243	    /// @notice Withdraws the accrued platform fee
244	    function claimPlatformFee() external onlyOwner {
...
272	    function _getRewardsSinceLastClaim(uint256 _id) internal view returns (uint256 amount) {
...
279	    /// @notice Splits the fee among the share holder, creator and platform
280	    function _splitFees(
281	        uint256 _id,
282	        uint256 _fee,
283	        uint256 _tokenCount
284	    ) internal {
```
[243..244](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L243-L244)
[272](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L272)
[279..284](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L279-L284)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
10	    constructor(uint256 _priceIncrease) {
...
14	    function getPriceAndFee(uint256 shareCount, uint256 amount)
15	        external
16	        view
17	        override
18	        returns (uint256 price, uint256 fee)
19	    {
...
27	    function getFee(uint256 shareCount) public pure override returns (uint256) {
...
38	    /// @dev Returns the log2 of `x`.
39	    /// Equivalent to computing the index of the most significant bit (MSB) of `x`.
40	    /// Returns 0 if `x` is zero.
41	    /// @notice Copied from Solady: https://github.com/Vectorized/solady/blob/main/src/utils/FixedPointMathLib.sol
42	    function log2(uint256 x) internal pure returns (uint256 r) {
```
[10](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L10)
[14..19](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L14-L19)
[27](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L27)
[38..42](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L38-L42)

---

	 - asD/src/asDFactory.sol

```solidity
33	    function create(string memory _name, string memory _symbol) external returns (address) {
```
[33](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L33)


</details>

-------
### [N-16] State variables should include comments
<a name="N-16"></a>
[To the top](#TOP)

Consider adding some comments on critical state variables to explain what they are supposed to do: this will help for future code reviews.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 6 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
...
15	    uint256 public constant HOLDER_CUT_BPS = 3_300; // 33%
...
16	    uint256 public constant CREATOR_CUT_BPS = 3_300; // 33%
```
[14](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L14)
[15](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L15)
[16](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L16)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
8	    uint256 public immutable priceIncrease;
```
[8](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L8)

---

	 - asD/src/asD.sol

```solidity
15	    address public immutable cNote; // Reference to the cNOTE token
```
[15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L15)

---

	 - asD/src/asDFactory.sol

```solidity
12	    address public immutable cNote;
```
[12](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L12)


</details>

-------
### [N-17] Duplicated `require/if` statements should be refactored
<a name="N-17"></a>
[To the top](#TOP)

These statements should be refactored to a separate function, as there are multiple parts of the codebase that use the same logic, to improve the code readability and reduce code duplication.



#### <ins>Proof Of Concept</ins>

<details>

<summary>see 5 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
216	        if (rewardsSinceLastClaim > 0) {
...
165	        if (rewardsSinceLastClaim > 0) {
```
[216](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L216)
[165](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L165)

---

	 - asD/src/asD.sol

```solidity
54	        require(returnCode == 0, "Error when minting");
...
64	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem-underlying
...
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
```
[54](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L54)
[64](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L64)
[86](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L86)


</details>

-------
### [N-18] Function declarations should have NatSpec descriptions
<a name="N-18"></a>
[To the top](#TOP)

---

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 5 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
272	    function _getRewardsSinceLastClaim(uint256 _id) internal view returns (uint256 amount) {
273	        uint256 lastClaimedValue = rewardsLastClaimedValue[_id][msg.sender];
274	        amount =
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
276	            1e18;
277	    }
```
[272..277](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L272-L277)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
10	    constructor(uint256 _priceIncrease) {
11	        priceIncrease = _priceIncrease;
12	    }
...
14	    function getPriceAndFee(uint256 shareCount, uint256 amount)
15	        external
16	        view
17	        override
18	        returns (uint256 price, uint256 fee)
19	    {
20	        for (uint256 i = shareCount; i < shareCount + amount; i++) {
21	            uint256 tokenPrice = priceIncrease * i;
22	            price += tokenPrice;
23	            fee += (getFee(i) * tokenPrice) / 1e18;
24	        }
25	    }
...
27	    function getFee(uint256 shareCount) public pure override returns (uint256) {
28	        uint256 divisor;
29	        if (shareCount > 1) {
30	            divisor = log2(shareCount);
31	        } else {
32	            divisor = 1;
33	        }
34	        // 0.1 / log2(shareCount)
35	        return 1e17 / divisor;
36	    }
```
[10..12](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L10-L12)
[14..25](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L14-L25)
[27..36](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L27-L36)

---

	 - asD/src/asDFactory.sol

```solidity
33	    function create(string memory _name, string memory _symbol) external returns (address) {
34	        asD createdToken = new asD(_name, _symbol, msg.sender, cNote, owner());
35	        isAsD[address(createdToken)] = true;
36	        emit CreatedToken(address(createdToken), _symbol, _name, msg.sender);
37	        return address(createdToken);
38	    }
```
[33..38](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L33-L38)


</details>

-------
### [N-19] Missing NatSpec from function definition
<a name="N-19"></a>
[To the top](#TOP)

Some functions miss a NatSpec, which should be a [best practice](https://docs.soliditylang.org/en/latest/natspec-format.html) to add as a documentation.

Even if Natspec for internal and private function is not parsed (but this may change in the future, according to the official docs), it still helps while reviewing the codebase.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 5 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
272	    function _getRewardsSinceLastClaim(uint256 _id) internal view returns (uint256 amount) {
```
[272](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L272)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
10	    constructor(uint256 _priceIncrease) {
...
14	    function getPriceAndFee(uint256 shareCount, uint256 amount)
15	        external
16	        view
17	        override
18	        returns (uint256 price, uint256 fee)
19	    {
...
27	    function getFee(uint256 shareCount) public pure override returns (uint256) {
```
[10](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L10)
[14..19](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L14-L19)
[27](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L27)

---

	 - asD/src/asDFactory.sol

```solidity
33	    function create(string memory _name, string memory _symbol) external returns (address) {
```
[33](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L33)


</details>

-------
### [N-20] Missing NatSpec `@notice` from function declaration
<a name="N-20"></a>
[To the top](#TOP)

Some functions have an incomplete NatSpec: add a `@notice` notation to describe the function to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 5 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
272	    function _getRewardsSinceLastClaim(uint256 _id) internal view returns (uint256 amount) {
```
[272](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L272)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
10	    constructor(uint256 _priceIncrease) {
...
14	    function getPriceAndFee(uint256 shareCount, uint256 amount)
15	        external
16	        view
17	        override
18	        returns (uint256 price, uint256 fee)
19	    {
...
27	    function getFee(uint256 shareCount) public pure override returns (uint256) {
```
[10](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L10)
[14..19](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L14-L19)
[27](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L27)

---

	 - asD/src/asDFactory.sol

```solidity
33	    function create(string memory _name, string memory _symbol) external returns (address) {
```
[33](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L33)


</details>

-------
### [N-21] Large multiples of ten should use scientific notation
<a name="N-21"></a>
[To the top](#TOP)

Use a scientific notation rather than decimal literals (e.g. `1e6` instead of `1000000`), for better code readability. The same for exponentiation  (e.g. `1e18` instead of `10**18`): although the compiler is capable of optimizing it, it is considered good coding practice to utilize idioms that don't rely on compiler optimization, whenever possible.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
...
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
...
285	        uint256 shareHolderFee = (_fee * HOLDER_CUT_BPS) / 10_000;
...
286	        uint256 shareCreatorFee = (_fee * CREATOR_CUT_BPS) / 10_000;
```
[14](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L14)
[197](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L197)
[285](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L285)
[286](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L286)


</details>

-------
### [N-22] Missing NatSpec from contract declarations
<a name="N-22"></a>
[To the top](#TOP)

Some contracts miss a `@dev`/`@notice` NatSpec, which should be a [best practice](https://docs.soliditylang.org/en/latest/natspec-format.html) to add as a documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
10	contract Market is ERC1155, Ownable2Step {
11	    /*//////////////////////////////////////////////////////////////
12	                                 CONSTANTS
13	    //////////////////////////////////////////////////////////////*/
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
```
[10..14](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L10-L14)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
6	contract LinearBondingCurve is IBondingCurve {
7	    // By how much the price increases per share, provided in the token decimals
8	    uint256 public immutable priceIncrease;
```
[6..8](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L6-L8)

---

	 - asD/src/asD.sol

```solidity
11	contract asD is ERC20, Ownable2Step {
12	    /*//////////////////////////////////////////////////////////////
13	                                 STATE
14	    //////////////////////////////////////////////////////////////*/
15	    address public immutable cNote; // Reference to the cNOTE token
```
[11..15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L11-L15)

---

	 - asD/src/asDFactory.sol

```solidity
8	contract asDFactory is Ownable2Step {
9	    /*//////////////////////////////////////////////////////////////
10	                                 STATE
11	    //////////////////////////////////////////////////////////////*/
12	    address public immutable cNote;
```
[8..12](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L8-L12)


</details>

-------
### [N-23] Missing NatSpec `@author` from contract declaration
<a name="N-23"></a>
[To the top](#TOP)

Some contract definitions have an incomplete NatSpec: add a `@author` notation to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
10	contract Market is ERC1155, Ownable2Step {
11	    /*//////////////////////////////////////////////////////////////
12	                                 CONSTANTS
13	    //////////////////////////////////////////////////////////////*/
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
```
[10..14](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L10-L14)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
6	contract LinearBondingCurve is IBondingCurve {
7	    // By how much the price increases per share, provided in the token decimals
8	    uint256 public immutable priceIncrease;
```
[6..8](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L6-L8)

---

	 - asD/src/asD.sol

```solidity
11	contract asD is ERC20, Ownable2Step {
12	    /*//////////////////////////////////////////////////////////////
13	                                 STATE
14	    //////////////////////////////////////////////////////////////*/
15	    address public immutable cNote; // Reference to the cNOTE token
```
[11..15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L11-L15)

---

	 - asD/src/asDFactory.sol

```solidity
8	contract asDFactory is Ownable2Step {
9	    /*//////////////////////////////////////////////////////////////
10	                                 STATE
11	    //////////////////////////////////////////////////////////////*/
12	    address public immutable cNote;
```
[8..12](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L8-L12)


</details>

-------
### [N-24] Missing NatSpec `@dev` from contract declaration
<a name="N-24"></a>
[To the top](#TOP)

Some contract definitions have an incomplete NatSpec: add a `@dev` notation to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
10	contract Market is ERC1155, Ownable2Step {
11	    /*//////////////////////////////////////////////////////////////
12	                                 CONSTANTS
13	    //////////////////////////////////////////////////////////////*/
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
```
[10..14](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L10-L14)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
6	contract LinearBondingCurve is IBondingCurve {
7	    // By how much the price increases per share, provided in the token decimals
8	    uint256 public immutable priceIncrease;
```
[6..8](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L6-L8)

---

	 - asD/src/asD.sol

```solidity
11	contract asD is ERC20, Ownable2Step {
12	    /*//////////////////////////////////////////////////////////////
13	                                 STATE
14	    //////////////////////////////////////////////////////////////*/
15	    address public immutable cNote; // Reference to the cNOTE token
```
[11..15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L11-L15)

---

	 - asD/src/asDFactory.sol

```solidity
8	contract asDFactory is Ownable2Step {
9	    /*//////////////////////////////////////////////////////////////
10	                                 STATE
11	    //////////////////////////////////////////////////////////////*/
12	    address public immutable cNote;
```
[8..12](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L8-L12)


</details>

-------
### [N-25] Missing NatSpec `@notice` from contract declaration
<a name="N-25"></a>
[To the top](#TOP)

Some contract definitions have an incomplete NatSpec: add a `@notice` notation to describe the contract to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
10	contract Market is ERC1155, Ownable2Step {
11	    /*//////////////////////////////////////////////////////////////
12	                                 CONSTANTS
13	    //////////////////////////////////////////////////////////////*/
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
```
[10..14](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L10-L14)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
6	contract LinearBondingCurve is IBondingCurve {
7	    // By how much the price increases per share, provided in the token decimals
8	    uint256 public immutable priceIncrease;
```
[6..8](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L6-L8)

---

	 - asD/src/asD.sol

```solidity
11	contract asD is ERC20, Ownable2Step {
12	    /*//////////////////////////////////////////////////////////////
13	                                 STATE
14	    //////////////////////////////////////////////////////////////*/
15	    address public immutable cNote; // Reference to the cNOTE token
```
[11..15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L11-L15)

---

	 - asD/src/asDFactory.sol

```solidity
8	contract asDFactory is Ownable2Step {
9	    /*//////////////////////////////////////////////////////////////
10	                                 STATE
11	    //////////////////////////////////////////////////////////////*/
12	    address public immutable cNote;
```
[8..12](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L8-L12)


</details>

-------
### [N-26] Missing NatSpec `@title` from contract declaration
<a name="N-26"></a>
[To the top](#TOP)

Some contract definitions have an incomplete NatSpec: add a `@title` notation to describe the contract to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
10	contract Market is ERC1155, Ownable2Step {
11	    /*//////////////////////////////////////////////////////////////
12	                                 CONSTANTS
13	    //////////////////////////////////////////////////////////////*/
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
```
[10..14](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L10-L14)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
6	contract LinearBondingCurve is IBondingCurve {
7	    // By how much the price increases per share, provided in the token decimals
8	    uint256 public immutable priceIncrease;
```
[6..8](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L6-L8)

---

	 - asD/src/asD.sol

```solidity
11	contract asD is ERC20, Ownable2Step {
12	    /*//////////////////////////////////////////////////////////////
13	                                 STATE
14	    //////////////////////////////////////////////////////////////*/
15	    address public immutable cNote; // Reference to the cNOTE token
```
[11..15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L11-L15)

---

	 - asD/src/asDFactory.sol

```solidity
8	contract asDFactory is Ownable2Step {
9	    /*//////////////////////////////////////////////////////////////
10	                                 STATE
11	    //////////////////////////////////////////////////////////////*/
12	    address public immutable cNote;
```
[8..12](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L8-L12)


</details>

-------
### [N-27] State variables not capped at reasonable values
<a name="N-27"></a>
[To the top](#TOP)

Consider adding minimum/maximum value checks to ensure that the state variables below can never be used to excessively harm users, including via griefing

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
150	    function buy(uint256 _id, uint256 _amount) external {
151	        require(shareData[_id].creator != msg.sender, "Creator cannot buy");
152	        (uint256 price, uint256 fee) = getBuyPrice(_id, _amount); // Reverts for non-existing ID
153	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), price + fee);
154	        // The reward calculation has to use the old rewards value (pre fee-split) to not include the fees of this buy
155	        // The rewardsLastClaimedValue then needs to be updated with the new value such that the user cannot claim fees of this buy
156	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
157	        // Split the fee among holder, creator and platform
158	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
159	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
160	
161	        shareData[_id].tokenCount += _amount;
162	        shareData[_id].tokensInCirculation += _amount;
163	        tokensByAddress[_id][msg.sender] += _amount;
164	
165	        if (rewardsSinceLastClaim > 0) {
166	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
167	        }
168	        emit SharesBought(_id, msg.sender, _amount, price, fee);
169	    }
...
174	    function sell(uint256 _id, uint256 _amount) external {
175	        (uint256 price, uint256 fee) = getSellPrice(_id, _amount);
176	        // Split the fee among holder, creator and platform
177	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
178	        // The user also gets the rewards of his own sale (which is not the case for buys)
179	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
180	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
181	
182	        shareData[_id].tokenCount -= _amount;
183	        shareData[_id].tokensInCirculation -= _amount;
184	        tokensByAddress[_id][msg.sender] -= _amount; // Would underflow if user did not have enough tokens
185	
186	        // Send the funds to the user
187	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim + price - fee);
188	        emit SharesSold(_id, msg.sender, _amount, price, fee);
189	    }
...
203	    function mintNFT(uint256 _id, uint256 _amount) external {
204	        uint256 fee = getNFTMintingPrice(_id, _amount);
205	
206	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
207	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
208	        // The user also gets the proportional rewards for the minting
209	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
210	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
211	        tokensByAddress[_id][msg.sender] -= _amount;
212	        shareData[_id].tokensInCirculation -= _amount;
213	
214	        _mint(msg.sender, _id, _amount, "");
215	
216	        if (rewardsSinceLastClaim > 0) {
217	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
218	        }
219	        // ERC1155 already logs, but we add this to have the price information
220	        emit NFTsCreated(_id, msg.sender, _amount, fee);
221	    }
...
226	    function burnNFT(uint256 _id, uint256 _amount) external {
227	        uint256 fee = getNFTMintingPrice(_id, _amount);
228	
229	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
230	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
231	        // The user does not get the proportional rewards for the burning (unless they have additional tokens that are not in the NFT)
232	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
233	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
234	        tokensByAddress[_id][msg.sender] += _amount;
235	        shareData[_id].tokensInCirculation += _amount;
236	        _burn(msg.sender, _id, _amount);
237	
238	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
239	        // ERC1155 already logs, but we add this to have the price information
240	        emit NFTsBurned(_id, msg.sender, _amount, fee);
241	    }
```
[150..169](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L150-L169)
[174..189](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L174-L189)
[203..221](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L203-L221)
[226..241](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L226-L241)


</details>

-------
### [N-28] Unused contract variables
<a name="N-28"></a>
[To the top](#TOP)

Note that there may be cases where a variable appears to be used, but this is only because there are multiple definitions of the varible in different files. In such cases, the variable definition should be moved into a separate file. The instances below are the unused variables.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 3 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
46	    mapping(uint256 => address) public shareBondingCurves;
```
[46](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L46)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
42	    function log2(uint256 x) internal pure returns (uint256 r) {
...
42	    function log2(uint256 x) internal pure returns (uint256 r) {
```
[42](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L42)
[42](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L42)


</details>

-------
### [N-29] `constructor` missing zero address check
<a name="N-29"></a>
[To the top](#TOP)

It is important to ensure that the constructor does not allow zero address to be set.
    This is a common mistake that can lead to loss of funds or redeployment of the contract.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 3 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
91	    constructor(string memory _uri, address _paymentToken) ERC1155(_uri) Ownable() {
92	        token = IERC20(_paymentToken);
93	        if (block.chainid == 7700 || block.chainid == 7701) {
94	            // Register CSR on Canto main- and testnet
95	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
96	            turnstile.register(tx.origin);
97	        }
98	    }
```
[91..98](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L91-L98)

---

	 - asD/src/asD.sol

```solidity
28	    constructor(
29	        string memory _name,
30	        string memory _symbol,
31	        address _owner,
32	        address _cNote,
33	        address _csrRecipient
34	    ) ERC20(_name, _symbol) {
35	        _transferOwnership(_owner);
36	        cNote = _cNote;
37	        if (block.chainid == 7700 || block.chainid == 7701) {
38	            // Register CSR on Canto main- and testnet
39	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
40	            turnstile.register(_csrRecipient);
41	        }
42	    }
```
[28..42](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L28-L42)

---

	 - asD/src/asDFactory.sol

```solidity
24	    constructor(address _cNote) {
25	        cNote = _cNote;
26	        if (block.chainid == 7700 || block.chainid == 7701) {
27	            // Register CSR on Canto main- and testnet
28	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
29	            turnstile.register(tx.origin);
30	        }
31	    }
```
[24..31](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L24-L31)


</details>

-------
### [N-30] Complex math should be split into multiple steps
<a name="N-30"></a>
[To the top](#TOP)

Consider splitting long arithmetic calculations into multiple steps to improve the code readability.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 3 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
...
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
276	            1e18;
```
[197](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L197)
[275..276](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L275-L276)

---

	 - asD/src/asD.sol

```solidity
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
77	            totalSupply();
```
[75..77](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L75-L77)


</details>

-------
### [N-31] Function state mutability can be restricted to view
<a name="N-31"></a>
[To the top](#TOP)

---

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 3 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
310	        require(whitelistedShareCreators[_address] != _isWhitelisted, "State already set");
311	        whitelistedShareCreators[_address] = _isWhitelisted;
312	    }
```
[309..312](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L309-L312)

---

	 - asD/src/asD.sol

```solidity
47	    function mint(uint256 _amount) external {
48	        CErc20Interface cNoteToken = CErc20Interface(cNote);
49	        IERC20 note = IERC20(cNoteToken.underlying());
50	        SafeERC20.safeTransferFrom(note, msg.sender, address(this), _amount);
51	        SafeERC20.safeApprove(note, cNote, _amount);
52	        uint256 returnCode = cNoteToken.mint(_amount);
53	        // Mint returns 0 on success: https://docs.compound.finance/v2/ctokens/#mint
54	        require(returnCode == 0, "Error when minting");
55	        _mint(msg.sender, _amount);
56	    }
...
60	    function burn(uint256 _amount) external {
61	        CErc20Interface cNoteToken = CErc20Interface(cNote);
62	        IERC20 note = IERC20(cNoteToken.underlying());
63	        uint256 returnCode = cNoteToken.redeemUnderlying(_amount); // Request _amount of NOTE (the underlying of cNOTE)
64	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem-underlying
65	        _burn(msg.sender, _amount);
66	        SafeERC20.safeTransfer(note, msg.sender, _amount);
67	    }
```
[47..56](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L47-L56)
[60..67](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L60-L67)


</details>

-------
### [N-32] Consider using `delete` rather than assigning zero/false to clear values
<a name="N-32"></a>
[To the top](#TOP)

The `delete` keyword more closely matches the semantics of what is being done, and draws more attention to the changing of state, which may lead to a more thorough audit of its associated logic

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
246	        platformPool = 0;
...
256	        shareData[_id].shareCreatorPool = 0;
```
[246](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L246)
[256](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L256)


</details>

-------
### [N-33] Consider moving `msg.sender` checks to a common authorization `modifier`
<a name="N-33"></a>
[To the top](#TOP)

---

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
151	        require(shareData[_id].creator != msg.sender, "Creator cannot buy");
...
254	        require(shareData[_id].creator == msg.sender, "Not creator");
```
[151](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L151)
[254](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L254)


</details>

-------
### [N-34] Use of `override` is unnecessary
<a name="N-34"></a>
[To the top](#TOP)

Starting with Solidity version [0.8.8](https://docs.soliditylang.org/en/v0.8.20/contracts.html#function-overriding), using the `override` keyword when the function solely overrides an interface function, and the function doesn't exist in multiple base contracts, is unnecessary.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
14	    function getPriceAndFee(uint256 shareCount, uint256 amount)
15	        external
16	        view
17	        override
18	        returns (uint256 price, uint256 fee)
19	    {
20	        for (uint256 i = shareCount; i < shareCount + amount; i++) {
21	            uint256 tokenPrice = priceIncrease * i;
22	            price += tokenPrice;
23	            fee += (getFee(i) * tokenPrice) / 1e18;
24	        }
25	    }
...
27	    function getFee(uint256 shareCount) public pure override returns (uint256) {
28	        uint256 divisor;
29	        if (shareCount > 1) {
30	            divisor = log2(shareCount);
31	        } else {
32	            divisor = 1;
33	        }
34	        // 0.1 / log2(shareCount)
35	        return 1e17 / divisor;
36	    }
```
[14..25](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L14-L25)
[27..36](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L27-L36)


</details>

-------
### [N-35] Missing event and or timelock for critical parameter change
<a name="N-35"></a>
[To the top](#TOP)

Events help non-contract tools to track changes, and timelocks prevent users from being surprised by changes

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
104	    function changeBondingCurveAllowed(address _bondingCurve, bool _newState) external onlyOwner {
105	        require(whitelistedBondingCurves[_bondingCurve] != _newState, "State already set");
106	        whitelistedBondingCurves[_bondingCurve] = _newState;
107	        emit BondingCurveStateChange(_bondingCurve, _newState);
108	    }
...
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
310	        require(whitelistedShareCreators[_address] != _isWhitelisted, "State already set");
311	        whitelistedShareCreators[_address] = _isWhitelisted;
312	    }
```
[104..108](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L104-L108)
[309..312](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L309-L312)


</details>

-------
### [N-36] Consider adding a block/deny-list
<a name="N-36"></a>
[To the top](#TOP)

Doing so will significantly increase centralization, but will help to prevent hackers from using stolen tokens

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
10	contract Market is ERC1155, Ownable2Step {
11	    /*//////////////////////////////////////////////////////////////
12	                                 CONSTANTS
13	    //////////////////////////////////////////////////////////////*/
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
15	    uint256 public constant HOLDER_CUT_BPS = 3_300; // 33%
16	    uint256 public constant CREATOR_CUT_BPS = 3_300; // 33%
17	    // Platform cut: 100% - HOLDER_CUT_BPS - CREATOR_CUT_BPS
18	
19	    /// @notice Payment token
20	    IERC20 public immutable token;
21	
22	    /*//////////////////////////////////////////////////////////////
23	                                 STATE
24	    //////////////////////////////////////////////////////////////*/
25	
26	    /// @notice Number of shares created
27	    uint256 public shareCount;
28	
29	    /// @notice Stores the share ID of a given share name
30	    mapping(string => uint256) public shareIDs;
31	
32	    struct ShareData {
33	        uint256 tokenCount; // Number of outstanding tokens
34	        uint256 tokensInCirculation; // Number of outstanding tokens - tokens that are minted as NFT, i.e. the number of tokens that receive fees
35	        uint256 shareHolderRewardsPerTokenScaled; // Accrued funds for the share holder per token, multiplied by 1e18 to avoid precision loss
36	        uint256 shareCreatorPool; // Unclaimed funds for the share creators
37	        address bondingCurve; // Bonding curve used for this share
38	        address creator; // Creator of the share
39	        string metadataURI; // URI of the metadata
40	    }
41	
42	    /// @notice Stores the data for a given share ID
43	    mapping(uint256 => ShareData) public shareData;
44	
45	    /// @notice Stores the bonding curve per share
46	    mapping(uint256 => address) public shareBondingCurves;
47	
48	    /// @notice Bonding curves that can be used for shares
49	    mapping(address => bool) public whitelistedBondingCurves;
50	
51	    /// @notice Stores the number of outstanding tokens per share and address
52	    mapping(uint256 => mapping(address => uint256)) public tokensByAddress;
53	
54	    /// @notice Value of ShareData.shareHolderRewardsPerTokenScaled at the last time a user claimed their rewards
55	    mapping(uint256 => mapping(address => uint256)) public rewardsLastClaimedValue;
56	
57	    /// @notice Unclaimed funds for the platform
58	    uint256 public platformPool;
59	
60	    /// @notice If true, only the whitelisted addresses can create shares
61	    bool public shareCreationRestricted = true;
62	
63	    /// @notice List of addresses that can add new shares when shareCreationRestricted is true
64	    mapping(address => bool) public whitelistedShareCreators;
65	
66	    /*//////////////////////////////////////////////////////////////
67	                                 EVENTS
68	    //////////////////////////////////////////////////////////////*/
69	    event BondingCurveStateChange(address indexed curve, bool isWhitelisted);
70	    event ShareCreated(uint256 indexed id, string name, address indexed bondingCurve, address indexed creator);
71	    event SharesBought(uint256 indexed id, address indexed buyer, uint256 amount, uint256 price, uint256 fee);
72	    event SharesSold(uint256 indexed id, address indexed seller, uint256 amount, uint256 price, uint256 fee);
73	    event NFTsCreated(uint256 indexed id, address indexed creator, uint256 amount, uint256 fee);
74	    event NFTsBurned(uint256 indexed id, address indexed burner, uint256 amount, uint256 fee);
75	    event PlatformFeeClaimed(address indexed claimer, uint256 amount);
76	    event CreatorFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
77	    event HolderFeeClaimed(address indexed claimer, uint256 indexed id, uint256 amount);
78	    event ShareCreationRestricted(bool isRestricted);
79	
80	    modifier onlyShareCreator() {
81	        require(
82	            !shareCreationRestricted || whitelistedShareCreators[msg.sender] || msg.sender == owner(),
83	            "Not allowed"
84	        );
85	        _;
86	    }
87	
88	    /// @notice Initiates CSR on main- and testnet
89	    /// @param _uri ERC1155 Base URI
90	    /// @param _paymentToken Address of the payment token
91	    constructor(string memory _uri, address _paymentToken) ERC1155(_uri) Ownable() {
92	        token = IERC20(_paymentToken);
93	        if (block.chainid == 7700 || block.chainid == 7701) {
94	            // Register CSR on Canto main- and testnet
95	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
96	            turnstile.register(tx.origin);
97	        }
98	    }
99	
100	    /// @notice Whitelist or remove whitelist for a bonding curve.
101	    /// @dev Whitelisting status is only checked when adding a share
102	    /// @param _bondingCurve Address of the bonding curve
103	    /// @param _newState True if whitelisted, false if not
104	    function changeBondingCurveAllowed(address _bondingCurve, bool _newState) external onlyOwner {
105	        require(whitelistedBondingCurves[_bondingCurve] != _newState, "State already set");
106	        whitelistedBondingCurves[_bondingCurve] = _newState;
107	        emit BondingCurveStateChange(_bondingCurve, _newState);
108	    }
109	
110	    /// @notice Creates a new share
111	    /// @param _shareName Name of the share
112	    /// @param _bondingCurve Address of the bonding curve, has to be whitelisted
113	    /// @param _metadataURI URI of the metadata
114	    function createNewShare(
115	        string memory _shareName,
116	        address _bondingCurve,
117	        string memory _metadataURI
118	    ) external onlyShareCreator returns (uint256 id) {
119	        require(whitelistedBondingCurves[_bondingCurve], "Bonding curve not whitelisted");
120	        require(shareIDs[_shareName] == 0, "Share already exists");
121	        id = ++shareCount;
122	        shareIDs[_shareName] = id;
123	        shareData[id].bondingCurve = _bondingCurve;
124	        shareData[id].creator = msg.sender;
125	        shareData[id].metadataURI = _metadataURI;
126	        emit ShareCreated(id, _shareName, _bondingCurve, msg.sender);
127	    }
128	
129	    /// @notice Returns the price and fee for buying a given number of shares.
130	    /// @param _id The ID of the share
131	    /// @param _amount The number of shares to buy.
132	    function getBuyPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
133	        // If id does not exist, this will return address(0), causing a revert in the next line
134	        address bondingCurve = shareData[_id].bondingCurve;
135	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount + 1, _amount);
136	    }
137	
138	    /// @notice Returns the price and fee for selling a given number of shares.
139	    /// @param _id The ID of the share
140	    /// @param _amount The number of shares to sell.
141	    function getSellPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
142	        // If id does not exist, this will return address(0), causing a revert in the next line
143	        address bondingCurve = shareData[_id].bondingCurve;
144	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount - _amount + 1, _amount);
145	    }
146	
147	    /// @notice Buy amount of tokens for a given share ID
148	    /// @param _id ID of the share
149	    /// @param _amount Amount of shares to buy
150	    function buy(uint256 _id, uint256 _amount) external {
151	        require(shareData[_id].creator != msg.sender, "Creator cannot buy");
152	        (uint256 price, uint256 fee) = getBuyPrice(_id, _amount); // Reverts for non-existing ID
153	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), price + fee);
154	        // The reward calculation has to use the old rewards value (pre fee-split) to not include the fees of this buy
155	        // The rewardsLastClaimedValue then needs to be updated with the new value such that the user cannot claim fees of this buy
156	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
157	        // Split the fee among holder, creator and platform
158	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
159	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
160	
161	        shareData[_id].tokenCount += _amount;
162	        shareData[_id].tokensInCirculation += _amount;
163	        tokensByAddress[_id][msg.sender] += _amount;
164	
165	        if (rewardsSinceLastClaim > 0) {
166	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
167	        }
168	        emit SharesBought(_id, msg.sender, _amount, price, fee);
169	    }
170	
171	    /// @notice Sell amount of tokens for a given share ID
172	    /// @param _id ID of the share
173	    /// @param _amount Amount of shares to sell
174	    function sell(uint256 _id, uint256 _amount) external {
175	        (uint256 price, uint256 fee) = getSellPrice(_id, _amount);
176	        // Split the fee among holder, creator and platform
177	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
178	        // The user also gets the rewards of his own sale (which is not the case for buys)
179	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
180	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
181	
182	        shareData[_id].tokenCount -= _amount;
183	        shareData[_id].tokensInCirculation -= _amount;
184	        tokensByAddress[_id][msg.sender] -= _amount; // Would underflow if user did not have enough tokens
185	
186	        // Send the funds to the user
187	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim + price - fee);
188	        emit SharesSold(_id, msg.sender, _amount, price, fee);
189	    }
190	
191	    /// @notice Returns the price and fee for minting a given number of NFTs.
192	    /// @param _id The ID of the share
193	    /// @param _amount The number of NFTs to mint.
194	    function getNFTMintingPrice(uint256 _id, uint256 _amount) public view returns (uint256 fee) {
195	        address bondingCurve = shareData[_id].bondingCurve;
196	        (uint256 priceForOne, ) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount, 1);
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
198	    }
199	
200	    /// @notice Convert amount of tokens to NFTs for a given share ID
201	    /// @param _id ID of the share
202	    /// @param _amount Amount of tokens to convert. User needs to have this many tokens.
203	    function mintNFT(uint256 _id, uint256 _amount) external {
204	        uint256 fee = getNFTMintingPrice(_id, _amount);
205	
206	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
207	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
208	        // The user also gets the proportional rewards for the minting
209	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
210	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
211	        tokensByAddress[_id][msg.sender] -= _amount;
212	        shareData[_id].tokensInCirculation -= _amount;
213	
214	        _mint(msg.sender, _id, _amount, "");
215	
216	        if (rewardsSinceLastClaim > 0) {
217	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
218	        }
219	        // ERC1155 already logs, but we add this to have the price information
220	        emit NFTsCreated(_id, msg.sender, _amount, fee);
221	    }
222	
223	    /// @notice Burn amount of NFTs for a given share ID to get back tokens
224	    /// @param _id ID of the share
225	    /// @param _amount Amount of NFTs to burn
226	    function burnNFT(uint256 _id, uint256 _amount) external {
227	        uint256 fee = getNFTMintingPrice(_id, _amount);
228	
229	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
230	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
231	        // The user does not get the proportional rewards for the burning (unless they have additional tokens that are not in the NFT)
232	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
233	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
234	        tokensByAddress[_id][msg.sender] += _amount;
235	        shareData[_id].tokensInCirculation += _amount;
236	        _burn(msg.sender, _id, _amount);
237	
238	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
239	        // ERC1155 already logs, but we add this to have the price information
240	        emit NFTsBurned(_id, msg.sender, _amount, fee);
241	    }
242	
243	    /// @notice Withdraws the accrued platform fee
244	    function claimPlatformFee() external onlyOwner {
245	        uint256 amount = platformPool;
246	        platformPool = 0;
247	        SafeERC20.safeTransfer(token, msg.sender, amount);
248	        emit PlatformFeeClaimed(msg.sender, amount);
249	    }
250	
251	    /// @notice Withdraws the accrued share creator fee
252	    /// @param _id ID of the share
253	    function claimCreatorFee(uint256 _id) external {
254	        require(shareData[_id].creator == msg.sender, "Not creator");
255	        uint256 amount = shareData[_id].shareCreatorPool;
256	        shareData[_id].shareCreatorPool = 0;
257	        SafeERC20.safeTransfer(token, msg.sender, amount);
258	        emit CreatorFeeClaimed(msg.sender, _id, amount);
259	    }
260	
261	    /// @notice Withdraws the accrued share holder fee
262	    /// @param _id ID of the share
263	    function claimHolderFee(uint256 _id) external {
264	        uint256 amount = _getRewardsSinceLastClaim(_id);
265	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
266	        if (amount > 0) {
267	            SafeERC20.safeTransfer(token, msg.sender, amount);
268	        }
269	        emit HolderFeeClaimed(msg.sender, _id, amount);
270	    }
271	
272	    function _getRewardsSinceLastClaim(uint256 _id) internal view returns (uint256 amount) {
273	        uint256 lastClaimedValue = rewardsLastClaimedValue[_id][msg.sender];
274	        amount =
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
276	            1e18;
277	    }
278	
279	    /// @notice Splits the fee among the share holder, creator and platform
280	    function _splitFees(
281	        uint256 _id,
282	        uint256 _fee,
283	        uint256 _tokenCount
284	    ) internal {
285	        uint256 shareHolderFee = (_fee * HOLDER_CUT_BPS) / 10_000;
286	        uint256 shareCreatorFee = (_fee * CREATOR_CUT_BPS) / 10_000;
287	        uint256 platformFee = _fee - shareHolderFee - shareCreatorFee;
288	        shareData[_id].shareCreatorPool += shareCreatorFee;
289	        if (_tokenCount > 0) {
290	            shareData[_id].shareHolderRewardsPerTokenScaled += (shareHolderFee * 1e18) / _tokenCount;
291	        } else {
292	            // If there are no tokens in circulation, the fee goes to the platform
293	            platformFee += shareHolderFee;
294	        }
295	        platformPool += platformFee;
296	    }
297	
298	    /// @notice Restricts or unrestricts share creation
299	    /// @param _isRestricted True if restricted, false if not
300	    function restrictShareCreation(bool _isRestricted) external onlyOwner {
301	        require(shareCreationRestricted != _isRestricted, "State already set");
302	        shareCreationRestricted = _isRestricted;
303	        emit ShareCreationRestricted(_isRestricted);
304	    }
305	
306	    /// @notice Adds or removes an address from the whitelist of share creators
307	    /// @param _address Address to add or remove
308	    /// @param _isWhitelisted True if whitelisted, false if not
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
310	        require(whitelistedShareCreators[_address] != _isWhitelisted, "State already set");
311	        whitelistedShareCreators[_address] = _isWhitelisted;
312	    }
313	}
```
[10..313](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L10-L313)

---

	 - asD/src/asD.sol

```solidity
11	contract asD is ERC20, Ownable2Step {
12	    /*//////////////////////////////////////////////////////////////
13	                                 STATE
14	    //////////////////////////////////////////////////////////////*/
15	    address public immutable cNote; // Reference to the cNOTE token
16	
17	    /*//////////////////////////////////////////////////////////////
18	                                 EVENTS
19	    //////////////////////////////////////////////////////////////*/
20	    event CarryWithdrawal(uint256 amount);
21	
22	    /// @notice Initiates CSR on main- and testnet
23	    /// @param _name Name of the token
24	    /// @param _symbol Symbol of the token
25	    /// @param _owner Initial owner of the vault/token
26	    /// @param _cNote Address of the cNOTE token
27	    /// @param _csrRecipient Address that should receive CSR rewards
28	    constructor(
29	        string memory _name,
30	        string memory _symbol,
31	        address _owner,
32	        address _cNote,
33	        address _csrRecipient
34	    ) ERC20(_name, _symbol) {
35	        _transferOwnership(_owner);
36	        cNote = _cNote;
37	        if (block.chainid == 7700 || block.chainid == 7701) {
38	            // Register CSR on Canto main- and testnet
39	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
40	            turnstile.register(_csrRecipient);
41	        }
42	    }
43	
44	    /// @notice Mint amount of asD tokens by providing NOTE. The NOTE:asD exchange rate is always 1:1
45	    /// @param _amount Amount of tokens to mint
46	    /// @dev User needs to approve the asD contract for _amount of NOTE
47	    function mint(uint256 _amount) external {
48	        CErc20Interface cNoteToken = CErc20Interface(cNote);
49	        IERC20 note = IERC20(cNoteToken.underlying());
50	        SafeERC20.safeTransferFrom(note, msg.sender, address(this), _amount);
51	        SafeERC20.safeApprove(note, cNote, _amount);
52	        uint256 returnCode = cNoteToken.mint(_amount);
53	        // Mint returns 0 on success: https://docs.compound.finance/v2/ctokens/#mint
54	        require(returnCode == 0, "Error when minting");
55	        _mint(msg.sender, _amount);
56	    }
57	
58	    /// @notice Burn amount of asD tokens to get back NOTE. Like when minting, the NOTE:asD exchange rate is always 1:1
59	    /// @param _amount Amount of tokens to burn
60	    function burn(uint256 _amount) external {
61	        CErc20Interface cNoteToken = CErc20Interface(cNote);
62	        IERC20 note = IERC20(cNoteToken.underlying());
63	        uint256 returnCode = cNoteToken.redeemUnderlying(_amount); // Request _amount of NOTE (the underlying of cNOTE)
64	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem-underlying
65	        _burn(msg.sender, _amount);
66	        SafeERC20.safeTransfer(note, msg.sender, _amount);
67	    }
68	
69	    /// @notice Withdraw the interest that accrued, only callable by the owner.
70	    /// @param _amount Amount of NOTE to withdraw. 0 for withdrawing the maximum possible amount
71	    /// @dev The function checks that the owner does not withdraw too much NOTE, i.e. that a 1:1 NOTE:asD exchange rate can be maintained after the withdrawal
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
73	        uint256 exchangeRate = CTokenInterface(cNote).exchangeRateCurrent(); // Scaled by 1 * 10^(18 - 8 + Underlying Token Decimals), i.e. 10^(28) in our case
74	        // The amount of cNOTE the contract has to hold (based on the current exchange rate which is always increasing) such that it is always possible to receive 1 NOTE when burning 1 asD
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
77	            totalSupply();
78	        if (_amount == 0) {
79	            _amount = maximumWithdrawable;
80	        } else {
81	            require(_amount <= maximumWithdrawable, "Too many tokens requested");
82	        }
83	        // Technically, _amount can still be 0 at this point, which would make the following two calls unnecessary.
84	        // But we do not handle this case specifically, as the only consequence is that the owner wastes a bit of gas when there is nothing to withdraw
85	        uint256 returnCode = CErc20Interface(cNote).redeemUnderlying(_amount);
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
87	        IERC20 note = IERC20(CErc20Interface(cNote).underlying());
88	        SafeERC20.safeTransfer(note, msg.sender, _amount);
89	        emit CarryWithdrawal(_amount);
90	    }
91	}
```
[11..91](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L11-L91)


</details>

-------
### [N-37] Functions which are either private or internal should have a preceding _ in their name
<a name="N-37"></a>
[To the top](#TOP)

Add a preceding underscore to the function name, take care to refactor where there functions are called

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
42	    function log2(uint256 x) internal pure returns (uint256 r) {
43	        /// @solidity memory-safe-assembly
44	        assembly {
45	            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
46	            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
47	            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
48	            r := or(r, shl(4, lt(0xffff, shr(r, x))))
49	            r := or(r, shl(3, lt(0xff, shr(r, x))))
50	            // forgefmt: disable-next-item
51	            r := or(
52	                r,
53	                byte(
54	                    and(0x1f, shr(shr(r, x), 0x8421084210842108cc6318c6db6d54be)),
55	                    0x0706060506020504060203020504030106050205030304010505030400000000
56	                )
57	            )
58	        }
59	    }
```
[42..59](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L42-L59)


</details>

-------
### [N-38] Emits without msg.sender parameter
<a name="N-38"></a>
[To the top](#TOP)

If msg.sender play a part in the functionality of a function, any emits of this function should include msg.sender to ensure transparency with users

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - asD/src/asD.sol

```solidity
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
73	        uint256 exchangeRate = CTokenInterface(cNote).exchangeRateCurrent(); // Scaled by 1 * 10^(18 - 8 + Underlying Token Decimals), i.e. 10^(28) in our case
74	        // The amount of cNOTE the contract has to hold (based on the current exchange rate which is always increasing) such that it is always possible to receive 1 NOTE when burning 1 asD
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
77	            totalSupply();
78	        if (_amount == 0) {
79	            _amount = maximumWithdrawable;
80	        } else {
81	            require(_amount <= maximumWithdrawable, "Too many tokens requested");
82	        }
83	        // Technically, _amount can still be 0 at this point, which would make the following two calls unnecessary.
84	        // But we do not handle this case specifically, as the only consequence is that the owner wastes a bit of gas when there is nothing to withdraw
85	        uint256 returnCode = CErc20Interface(cNote).redeemUnderlying(_amount);
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
87	        IERC20 note = IERC20(CErc20Interface(cNote).underlying());
88	        SafeERC20.safeTransfer(note, msg.sender, _amount);
89	        emit CarryWithdrawal(_amount);
90	    }
```
[72..90](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L72-L90)


</details>

-------
### [N-39] `if`-statement can be converted to a ternary
<a name="N-39"></a>
[To the top](#TOP)

The code can be made more compact while also increasing readability by converting the following `if`-statements to ternaries (e.g. `foo += (x > y) ? a : b`)

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
29	        if (shareCount > 1) {
30	            divisor = log2(shareCount);
31	        } else {
32	            divisor = 1;
33	        }
```
[29..33](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L29-L33)


</details>

-------
### [N-40] External calls in an un-bounded `for`-loop may result in a DOS
<a name="N-40"></a>
[To the top](#TOP)

Consider limiting the number of iterations in `for`-loops that make external calls

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
20	        for (uint256 i = shareCount; i < shareCount + amount; i++) {
21	            uint256 tokenPrice = priceIncrease * i;
22	            price += tokenPrice;
23	            fee += (getFee(i) * tokenPrice) / 1e18;
24	        }
```
[20..24](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L20-L24)


</details>

-------
### [N-41] Consider implementing two-step procedure for updating protocol addresses
<a name="N-41"></a>
[To the top](#TOP)

A copy-paste error or a typo may end up bricking protocol functionality, or sending tokens to an address with no known private key. Consider implementing a two-step procedure for updating protocol addresses, where the recipient is set as pending, and must 'accept' the assignment by making an affirmative call. A straight forward way of doing this would be to have the target contracts implement [EIP-165](https://eips.ethereum.org/EIPS/eip-165), and to have the 'set' functions ensure that the recipient is of the right interface type.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
114	    function createNewShare(
115	        string memory _shareName,
116	        address _bondingCurve,
117	        string memory _metadataURI
118	    ) external onlyShareCreator returns (uint256 id) {
119	        require(whitelistedBondingCurves[_bondingCurve], "Bonding curve not whitelisted");
120	        require(shareIDs[_shareName] == 0, "Share already exists");
121	        id = ++shareCount;
122	        shareIDs[_shareName] = id;
123	        shareData[id].bondingCurve = _bondingCurve;
124	        shareData[id].creator = msg.sender;
125	        shareData[id].metadataURI = _metadataURI;
126	        emit ShareCreated(id, _shareName, _bondingCurve, msg.sender);
127	    }
```
[114..127](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L114-L127)


</details>

-------
### [N-42] Events that mark critical parameter changes should contain both the old and the new value
<a name="N-42"></a>
[To the top](#TOP)

This should especially be done if the new value is not required to be different from the old value

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
69	    event BondingCurveStateChange(address indexed curve, bool isWhitelisted);
```
[69](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L69)


</details>

-------
### [N-43] Memory-safe annotation preferred over comment variant
<a name="N-43"></a>
[To the top](#TOP)

The memory-safe annotation (`assembly ("memory-safe") { ... }`), available starting in Solidity version 0.8.13 is preferred over the comment variant, which will be removed in a future breaking [release](https://docs.soliditylang.org/en/v0.8.13/assembly.html#memory-safety). The comment variant is only meant for externalized library code that needs to work in earlier versions (e.g. `SafeTransferLib` needs to be able to be used in many different versions).

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
44	        assembly {
45	            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
46	            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
47	            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
48	            r := or(r, shl(4, lt(0xffff, shr(r, x))))
49	            r := or(r, shl(3, lt(0xff, shr(r, x))))
50	            // forgefmt: disable-next-item
51	            r := or(
52	                r,
53	                byte(
54	                    and(0x1f, shr(shr(r, x), 0x8421084210842108cc6318c6db6d54be)),
55	                    0x0706060506020504060203020504030106050205030304010505030400000000
56	                )
57	            )
58	        }
```
[44..58](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L44-L58)


</details>

-------
### [N-44] Unused function parameter
<a name="N-44"></a>
[To the top](#TOP)

Comment out the variable name to suppress compiler warnings

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
42	    function log2(uint256 x) internal pure returns (uint256 r) {
43	        /// @solidity memory-safe-assembly
44	        assembly {
45	            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
46	            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
47	            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
48	            r := or(r, shl(4, lt(0xffff, shr(r, x))))
49	            r := or(r, shl(3, lt(0xff, shr(r, x))))
50	            // forgefmt: disable-next-item
51	            r := or(
52	                r,
53	                byte(
54	                    and(0x1f, shr(shr(r, x), 0x8421084210842108cc6318c6db6d54be)),
55	                    0x0706060506020504060203020504030106050205030304010505030400000000
56	                )
57	            )
58	        }
59	    }
```
[42..59](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L42-L59)


</details>

-------
### [N-45] Modifier declarations should have NatSpec descriptions
<a name="N-45"></a>
[To the top](#TOP)

---

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
80	    modifier onlyShareCreator() {
81	        require(
82	            !shareCreationRestricted || whitelistedShareCreators[msg.sender] || msg.sender == owner(),
83	            "Not allowed"
84	        );
85	        _;
86	    }
```
[80..86](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L80-L86)


</details>

-------
### [N-46] Missing NatSpec from modifiers definitions
<a name="N-46"></a>
[To the top](#TOP)

Consider adding some comments on modifier declarations to explain what they are supposed to do: this will help for future code reviews.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
80	    modifier onlyShareCreator() {
```
[80](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L80)


</details>

-------
### [N-47] Missing NatSpec `@dev` from modifier declaration
<a name="N-47"></a>
[To the top](#TOP)

Some modifiers have an incomplete NatSpec: add a `@dev` notation to describe the modifier to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
80	    modifier onlyShareCreator() {
```
[80](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L80)


</details>

-------
### [N-48] Missing NatSpec `@notice` from modifier declaration
<a name="N-48"></a>
[To the top](#TOP)

Some modifiers have an incomplete NatSpec: add a `@notice` notation to describe the modifier to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
80	    modifier onlyShareCreator() {
```
[80](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L80)


</details>

-------
### [N-49] Missing NatSpec `@param` from modifier declaration
<a name="N-49"></a>
[To the top](#TOP)

Some modifiers have an incomplete NatSpec: add a `@param` notation to describe the modifier parameters to improve the code documentation.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
80	    modifier onlyShareCreator() {
```
[80](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L80)


</details>

-------
### [N-50] Contracts should have full test coverage
<a name="N-50"></a>
[To the top](#TOP)

A 100% test coverage is not foolproof, but it helps immensely in reducing the amount of bugs that may occur.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>

Placeholder, for non-solidity issues.


[C:/Users/pasha/code/arena4bot/project/2023-11-canto](https://github.com/code-423n4/2023-11-canto/blob/main/)


</details>

-------
### [N-51] Large or complicated code bases should implement invariant tests
<a name="N-51"></a>
[To the top](#TOP)

This includes: large code bases, or code with lots of inline-assembly, complicated math, or complicated interactions between multiple contracts.

Invariant fuzzers such as Echidna require the test writer to come up with invariants which should not be violated under any circumstances, and the fuzzer tests various inputs and function calls to ensure that the invariants always hold.

Even code with 100% code coverage can still have bugs due to the order of the operations a user performs, and invariant fuzzers may help significantly.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>

Placeholder, for non-solidity issues.


[C:/Users/pasha/code/arena4bot/project/2023-11-canto](https://github.com/code-423n4/2023-11-canto/blob/main/)


</details>

-------
### [N-52] Codebase should implement formal verification testing
<a name="N-52"></a>
[To the top](#TOP)

Formal verification is the act of proving or disproving the correctness of intended algorithms underlying a system with respect to a certain formal specification/property/invariant, using formal methods of mathematics.

Some tools that are currently available to perform these tests on smart contracts are [SMTChecker](https://docs.soliditylang.org/en/latest/smtchecker.html) and [Certora Prover](https://www.certora.com/).

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>

Placeholder, for non-solidity issues.


[C:/Users/pasha/code/arena4bot/project/2023-11-canto](https://github.com/code-423n4/2023-11-canto/blob/main/)


</details>


## Gas Risk Issues

### [G-01] State variables should be cached in stack variables rather than re-reading them from storage
<a name="G-01"></a>
[To the top](#TOP)

The instances below point to the second+ access of a state variable within a function. Caching of a state variable replaces each Gwarmaccess (**100 gas**) with a much cheaper stack read. Other less obvious fixes/optimizations include having local memory caches of state variable structs, or having local caches of state variable contracts/addresses.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 21 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
134	        address bondingCurve = shareData[_id].bondingCurve;
...
135	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount + 1, _amount);
...
143	        address bondingCurve = shareData[_id].bondingCurve;
...
144	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount - _amount + 1, _amount);
...
151	        require(shareData[_id].creator != msg.sender, "Creator cannot buy");
...
158	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
...
159	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
...
177	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
...
180	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
...
195	        address bondingCurve = shareData[_id].bondingCurve;
...
196	        (uint256 priceForOne, ) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount, 1);
...
207	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
...
210	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
...
230	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
...
233	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
...
254	        require(shareData[_id].creator == msg.sender, "Not creator");
...
255	        uint256 amount = shareData[_id].shareCreatorPool;
...
265	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
...
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
...
105	        require(whitelistedBondingCurves[_bondingCurve] != _newState, "State already set");
...
119	        require(whitelistedBondingCurves[_bondingCurve], "Bonding curve not whitelisted");
```
[134](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L134)
[135](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L135)
[143](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L143)
[144](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L144)
[151](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L151)
[158](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L158)
[159](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L159)
[177](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L177)
[180](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L180)
[195](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L195)
[196](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L196)
[207](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L207)
[210](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L210)
[230](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L230)
[233](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L233)
[254](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L254)
[255](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L255)
[265](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L265)
[275](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L275)
[105](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L105)
[119](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L119)


</details>

-------
### [G-02] Avoid Unnecessary Public Variables
<a name="G-02"></a>
[To the top](#TOP)

Public storage variables increase the contract's size due to the implicit generation of public getter functions. 
This makes the contract larger and could increase deployment and interaction costs.

If you do not require other contracts to read these variables, consider making them `private` or `internal`. 

Example:
```solidity
/// 145426 gas to deploy
contract PublicState {
  address public first;
  address public second;
}
/// 77126 gas to deploy
contract PrivateState {
  address private first;
  address private second;
}
```

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 18 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
...
15	    uint256 public constant HOLDER_CUT_BPS = 3_300; // 33%
...
16	    uint256 public constant CREATOR_CUT_BPS = 3_300; // 33%
...
20	    IERC20 public immutable token;
...
27	    uint256 public shareCount;
...
30	    mapping(string => uint256) public shareIDs;
...
43	    mapping(uint256 => ShareData) public shareData;
...
46	    mapping(uint256 => address) public shareBondingCurves;
...
49	    mapping(address => bool) public whitelistedBondingCurves;
...
52	    mapping(uint256 => mapping(address => uint256)) public tokensByAddress;
...
55	    mapping(uint256 => mapping(address => uint256)) public rewardsLastClaimedValue;
...
58	    uint256 public platformPool;
...
61	    bool public shareCreationRestricted = true;
...
64	    mapping(address => bool) public whitelistedShareCreators;
```
[14](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L14)
[15](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L15)
[16](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L16)
[20](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L20)
[27](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L27)
[30](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L30)
[43](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L43)
[46](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L46)
[49](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L49)
[52](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L52)
[55](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L55)
[58](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L58)
[61](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L61)
[64](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L64)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
8	    uint256 public immutable priceIncrease;
```
[8](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L8)

---

	 - asD/src/asD.sol

```solidity
15	    address public immutable cNote; // Reference to the cNOTE token
```
[15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L15)

---

	 - asD/src/asDFactory.sol

```solidity
12	    address public immutable cNote;
...
15	    mapping(address => bool) public isAsD;
```
[12](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L12)
[15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L15)


</details>

-------
### [G-03] Consider Using Solady's Gas Optimized Lib for Math
<a name="G-03"></a>
[To the top](#TOP)

Utilizing gas-optimized math functions from libraries like [Solady](https://github.com/Vectorized/solady/blob/main/src/utils/FixedPointMathLib.sol) can lead to more efficient smart contracts.
This is particularly beneficial in contracts where these operations are frequently used.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 17 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
...
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
...
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
...
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
276	            1e18;
...
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
...
285	        uint256 shareHolderFee = (_fee * HOLDER_CUT_BPS) / 10_000;
...
285	        uint256 shareHolderFee = (_fee * HOLDER_CUT_BPS) / 10_000;
...
286	        uint256 shareCreatorFee = (_fee * CREATOR_CUT_BPS) / 10_000;
...
286	        uint256 shareCreatorFee = (_fee * CREATOR_CUT_BPS) / 10_000;
...
290	            shareData[_id].shareHolderRewardsPerTokenScaled += (shareHolderFee * 1e18) / _tokenCount;
...
290	            shareData[_id].shareHolderRewardsPerTokenScaled += (shareHolderFee * 1e18) / _tokenCount;
```
[197](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L197)
[197](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L197)
[197](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L197)
[275..276](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L275-L276)
[275](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L275)
[285](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L285)
[285](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L285)
[286](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L286)
[286](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L286)
[290](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L290)
[290](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L290)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
21	            uint256 tokenPrice = priceIncrease * i;
...
23	            fee += (getFee(i) * tokenPrice) / 1e18;
...
23	            fee += (getFee(i) * tokenPrice) / 1e18;
...
35	        return 1e17 / divisor;
```
[21](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L21)
[23](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L23)
[23](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L23)
[35](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L35)

---

	 - asD/src/asD.sol

```solidity
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
...
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
```
[75..76](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L75-L76)
[75](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L75)


</details>

-------
### [G-04] Optimize External Calls with Assembly for Memory Efficiency
<a name="G-04"></a>
[To the top](#TOP)

Using interfaces to make external contract calls in Solidity is convenient but can be inefficient in terms of memory utilization.
Each such call involves creating a new memory location to store the data being passed, thus incurring memory expansion costs. 

Inline assembly allows for optimized memory usage by re-using already allocated memory spaces or using the scratch space for smaller datasets.
This can result in notable gas savings, especially for contracts that make frequent external calls.

Additionally, using inline assembly enables important safety checks like verifying if the target address has code deployed to it using `extcodesize(addr)` before making the call, mitigating risks associated with contract interactions.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 14 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
96	            turnstile.register(tx.origin);
...
135	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount + 1, _amount);
...
144	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount - _amount + 1, _amount);
...
196	        (uint256 priceForOne, ) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount, 1);
```
[96](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L96)
[135](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L135)
[144](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L144)
[196](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L196)

---

	 - asD/src/asD.sol

```solidity
40	            turnstile.register(_csrRecipient);
...
49	        IERC20 note = IERC20(cNoteToken.underlying());
...
52	        uint256 returnCode = cNoteToken.mint(_amount);
...
62	        IERC20 note = IERC20(cNoteToken.underlying());
...
63	        uint256 returnCode = cNoteToken.redeemUnderlying(_amount); // Request _amount of NOTE (the underlying of cNOTE)
...
73	        uint256 exchangeRate = CTokenInterface(cNote).exchangeRateCurrent(); // Scaled by 1 * 10^(18 - 8 + Underlying Token Decimals), i.e. 10^(28) in our case
...
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
...
85	        uint256 returnCode = CErc20Interface(cNote).redeemUnderlying(_amount);
...
87	        IERC20 note = IERC20(CErc20Interface(cNote).underlying());
```
[40](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L40)
[49](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L49)
[52](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L52)
[62](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L62)
[63](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L63)
[73](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L73)
[75](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L75)
[85](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L85)
[87](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L87)

---

	 - asD/src/asDFactory.sol

```solidity
29	            turnstile.register(tx.origin);
```
[29](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L29)


</details>

-------
### [G-05] Consider Caching Multiple Accesses to Mappings/Arrays
<a name="G-05"></a>
[To the top](#TOP)

Leveraging a local variable to cache these values when accessed more than once can yield a gas saving of approximately 42 units per access. This reduction is attributed to eliminating the need for recalculating the key's keccak256 hash (which costs Gkeccak256 - 30 gas) and the associated stack operations. For arrays, this also prevents the overhead of re-computing offsets in memory or calldata.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 13 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
104	    function changeBondingCurveAllowed(address _bondingCurve, bool _newState) external onlyOwner {
105	        require(whitelistedBondingCurves[_bondingCurve] != _newState, "State already set");
106	        whitelistedBondingCurves[_bondingCurve] = _newState;
107	        emit BondingCurveStateChange(_bondingCurve, _newState);
108	    }
...
114	    function createNewShare(
115	        string memory _shareName,
116	        address _bondingCurve,
117	        string memory _metadataURI
118	    ) external onlyShareCreator returns (uint256 id) {
119	        require(whitelistedBondingCurves[_bondingCurve], "Bonding curve not whitelisted");
120	        require(shareIDs[_shareName] == 0, "Share already exists");
121	        id = ++shareCount;
122	        shareIDs[_shareName] = id;
123	        shareData[id].bondingCurve = _bondingCurve;
124	        shareData[id].creator = msg.sender;
125	        shareData[id].metadataURI = _metadataURI;
126	        emit ShareCreated(id, _shareName, _bondingCurve, msg.sender);
127	    }
...
132	    function getBuyPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
133	        // If id does not exist, this will return address(0), causing a revert in the next line
134	        address bondingCurve = shareData[_id].bondingCurve;
135	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount + 1, _amount);
136	    }
...
141	    function getSellPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
142	        // If id does not exist, this will return address(0), causing a revert in the next line
143	        address bondingCurve = shareData[_id].bondingCurve;
144	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount - _amount + 1, _amount);
145	    }
...
150	    function buy(uint256 _id, uint256 _amount) external {
151	        require(shareData[_id].creator != msg.sender, "Creator cannot buy");
152	        (uint256 price, uint256 fee) = getBuyPrice(_id, _amount); // Reverts for non-existing ID
153	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), price + fee);
154	        // The reward calculation has to use the old rewards value (pre fee-split) to not include the fees of this buy
155	        // The rewardsLastClaimedValue then needs to be updated with the new value such that the user cannot claim fees of this buy
156	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
157	        // Split the fee among holder, creator and platform
158	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
159	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
160	
161	        shareData[_id].tokenCount += _amount;
162	        shareData[_id].tokensInCirculation += _amount;
163	        tokensByAddress[_id][msg.sender] += _amount;
164	
165	        if (rewardsSinceLastClaim > 0) {
166	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
167	        }
168	        emit SharesBought(_id, msg.sender, _amount, price, fee);
169	    }
...
174	    function sell(uint256 _id, uint256 _amount) external {
175	        (uint256 price, uint256 fee) = getSellPrice(_id, _amount);
176	        // Split the fee among holder, creator and platform
177	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
178	        // The user also gets the rewards of his own sale (which is not the case for buys)
179	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
180	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
181	
182	        shareData[_id].tokenCount -= _amount;
183	        shareData[_id].tokensInCirculation -= _amount;
184	        tokensByAddress[_id][msg.sender] -= _amount; // Would underflow if user did not have enough tokens
185	
186	        // Send the funds to the user
187	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim + price - fee);
188	        emit SharesSold(_id, msg.sender, _amount, price, fee);
189	    }
...
194	    function getNFTMintingPrice(uint256 _id, uint256 _amount) public view returns (uint256 fee) {
195	        address bondingCurve = shareData[_id].bondingCurve;
196	        (uint256 priceForOne, ) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount, 1);
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
198	    }
...
203	    function mintNFT(uint256 _id, uint256 _amount) external {
204	        uint256 fee = getNFTMintingPrice(_id, _amount);
205	
206	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
207	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
208	        // The user also gets the proportional rewards for the minting
209	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
210	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
211	        tokensByAddress[_id][msg.sender] -= _amount;
212	        shareData[_id].tokensInCirculation -= _amount;
213	
214	        _mint(msg.sender, _id, _amount, "");
215	
216	        if (rewardsSinceLastClaim > 0) {
217	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
218	        }
219	        // ERC1155 already logs, but we add this to have the price information
220	        emit NFTsCreated(_id, msg.sender, _amount, fee);
221	    }
...
226	    function burnNFT(uint256 _id, uint256 _amount) external {
227	        uint256 fee = getNFTMintingPrice(_id, _amount);
228	
229	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
230	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
231	        // The user does not get the proportional rewards for the burning (unless they have additional tokens that are not in the NFT)
232	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
233	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
234	        tokensByAddress[_id][msg.sender] += _amount;
235	        shareData[_id].tokensInCirculation += _amount;
236	        _burn(msg.sender, _id, _amount);
237	
238	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
239	        // ERC1155 already logs, but we add this to have the price information
240	        emit NFTsBurned(_id, msg.sender, _amount, fee);
241	    }
...
253	    function claimCreatorFee(uint256 _id) external {
254	        require(shareData[_id].creator == msg.sender, "Not creator");
255	        uint256 amount = shareData[_id].shareCreatorPool;
256	        shareData[_id].shareCreatorPool = 0;
257	        SafeERC20.safeTransfer(token, msg.sender, amount);
258	        emit CreatorFeeClaimed(msg.sender, _id, amount);
259	    }
...
272	    function _getRewardsSinceLastClaim(uint256 _id) internal view returns (uint256 amount) {
273	        uint256 lastClaimedValue = rewardsLastClaimedValue[_id][msg.sender];
274	        amount =
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
276	            1e18;
277	    }
...
280	    function _splitFees(
281	        uint256 _id,
282	        uint256 _fee,
283	        uint256 _tokenCount
284	    ) internal {
285	        uint256 shareHolderFee = (_fee * HOLDER_CUT_BPS) / 10_000;
286	        uint256 shareCreatorFee = (_fee * CREATOR_CUT_BPS) / 10_000;
287	        uint256 platformFee = _fee - shareHolderFee - shareCreatorFee;
288	        shareData[_id].shareCreatorPool += shareCreatorFee;
289	        if (_tokenCount > 0) {
290	            shareData[_id].shareHolderRewardsPerTokenScaled += (shareHolderFee * 1e18) / _tokenCount;
291	        } else {
292	            // If there are no tokens in circulation, the fee goes to the platform
293	            platformFee += shareHolderFee;
294	        }
295	        platformPool += platformFee;
296	    }
...
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
310	        require(whitelistedShareCreators[_address] != _isWhitelisted, "State already set");
311	        whitelistedShareCreators[_address] = _isWhitelisted;
312	    }
```
[104..108](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L104-L108)
[114..127](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L114-L127)
[132..136](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L132-L136)
[141..145](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L141-L145)
[150..169](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L150-L169)
[174..189](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L174-L189)
[194..198](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L194-L198)
[203..221](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L203-L221)
[226..241](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L226-L241)
[253..259](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L253-L259)
[272..277](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L272-L277)
[280..296](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L280-L296)
[309..312](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L309-L312)


</details>

-------
### [G-06] Use Custom Errors
<a name="G-06"></a>
[To the top](#TOP)

[Source](https://blog.soliditylang.org/2061/04/21/custom-errors/)
Instead of using error strings, to reduce deployment and runtime cost, you should use Custom Errors. This would save both deployment and runtime cost.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 12 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
81	        require(
82	            !shareCreationRestricted || whitelistedShareCreators[msg.sender] || msg.sender == owner(),
83	            "Not allowed"
84	        );
...
105	        require(whitelistedBondingCurves[_bondingCurve] != _newState, "State already set");
...
119	        require(whitelistedBondingCurves[_bondingCurve], "Bonding curve not whitelisted");
...
120	        require(shareIDs[_shareName] == 0, "Share already exists");
...
151	        require(shareData[_id].creator != msg.sender, "Creator cannot buy");
...
254	        require(shareData[_id].creator == msg.sender, "Not creator");
...
301	        require(shareCreationRestricted != _isRestricted, "State already set");
...
310	        require(whitelistedShareCreators[_address] != _isWhitelisted, "State already set");
```
[81..84](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L81-L84)
[105](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L105)
[119](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L119)
[120](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L120)
[151](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L151)
[254](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L254)
[301](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L301)
[310](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L310)

---

	 - asD/src/asD.sol

```solidity
54	        require(returnCode == 0, "Error when minting");
...
64	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem-underlying
...
81	            require(_amount <= maximumWithdrawable, "Too many tokens requested");
...
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
```
[54](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L54)
[64](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L64)
[81](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L81)
[86](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L86)


</details>

-------
### [G-07] Consider using `bytes32` rather than a `string`
<a name="G-07"></a>
[To the top](#TOP)

Using the `bytes` types for fixed-length strings is more efficient than having the EVM have to incur the overhead of string processing. Consider whether the value _needs_ to be a `string`. A good reason to keep it as a `string` would be if the variable is defined in an interface that this project does not own.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 12 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
30	    mapping(string => uint256) public shareIDs;
...
39	        string metadataURI; // URI of the metadata
...
70	    event ShareCreated(uint256 indexed id, string name, address indexed bondingCurve, address indexed creator);
...
91	    constructor(string memory _uri, address _paymentToken) ERC1155(_uri) Ownable() {
...
115	        string memory _shareName,
...
117	        string memory _metadataURI
```
[30](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L30)
[39](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L39)
[70](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L70)
[91](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L91)
[115](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L115)
[117](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L117)

---

	 - asD/src/asD.sol

```solidity
29	        string memory _name,
...
30	        string memory _symbol,
```
[29](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L29)
[30](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L30)

---

	 - asD/src/asDFactory.sol

```solidity
20	    event CreatedToken(address token, string symbol, string name, address creator);
...
20	    event CreatedToken(address token, string symbol, string name, address creator);
...
33	    function create(string memory _name, string memory _symbol) external returns (address) {
...
33	    function create(string memory _name, string memory _symbol) external returns (address) {
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L20)
[20](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L20)
[33](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L33)
[33](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L33)


</details>

-------
### [G-08] Use Assembly for Efficient Event Emission
<a name="G-08"></a>
[To the top](#TOP)

To efficiently emit events, consider utilizing assembly by making use of scratch space and the free memory pointer.
This approach can potentially avoid the costs associated with memory expansion.

However, it's crucial to cache and restore the free memory pointer for safe optimization.
Good examples of such practices can be found in well-optimized [Solady's codebases](https://github.com/Vectorized/solady/blob/main/src/tokens/ERC1155.sol#L167).
Please review your code and consider the potential gas savings of this approach.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 12 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
107	        emit BondingCurveStateChange(_bondingCurve, _newState);
...
126	        emit ShareCreated(id, _shareName, _bondingCurve, msg.sender);
...
168	        emit SharesBought(_id, msg.sender, _amount, price, fee);
...
188	        emit SharesSold(_id, msg.sender, _amount, price, fee);
...
220	        emit NFTsCreated(_id, msg.sender, _amount, fee);
...
240	        emit NFTsBurned(_id, msg.sender, _amount, fee);
...
248	        emit PlatformFeeClaimed(msg.sender, amount);
...
258	        emit CreatorFeeClaimed(msg.sender, _id, amount);
...
269	        emit HolderFeeClaimed(msg.sender, _id, amount);
...
303	        emit ShareCreationRestricted(_isRestricted);
```
[107](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L107)
[126](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L126)
[168](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L168)
[188](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L188)
[220](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L220)
[240](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L240)
[248](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L248)
[258](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L258)
[269](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L269)
[303](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L303)

---

	 - asD/src/asD.sol

```solidity
89	        emit CarryWithdrawal(_amount);
```
[89](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L89)

---

	 - asD/src/asDFactory.sol

```solidity
36	        emit CreatedToken(address(createdToken), _symbol, _name, msg.sender);
```
[36](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L36)


</details>

-------
### [G-09] Stack variable used as a cheaper cache for a state variable is only used once
<a name="G-09"></a>
[To the top](#TOP)

If the variable is only accessed once, it's cheaper to use the state variable directly that one time, and save the 3 gas the extra stack assignment would spend

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 9 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
132	    function getBuyPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
133	        // If id does not exist, this will return address(0), causing a revert in the next line
134	        address bondingCurve = shareData[_id].bondingCurve;
135	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount + 1, _amount);
136	    }
...
141	    function getSellPrice(uint256 _id, uint256 _amount) public view returns (uint256 price, uint256 fee) {
142	        // If id does not exist, this will return address(0), causing a revert in the next line
143	        address bondingCurve = shareData[_id].bondingCurve;
144	        (price, fee) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount - _amount + 1, _amount);
145	    }
...
174	    function sell(uint256 _id, uint256 _amount) external {
175	        (uint256 price, uint256 fee) = getSellPrice(_id, _amount);
176	        // Split the fee among holder, creator and platform
177	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
178	        // The user also gets the rewards of his own sale (which is not the case for buys)
179	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
180	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
181	
182	        shareData[_id].tokenCount -= _amount;
183	        shareData[_id].tokensInCirculation -= _amount;
184	        tokensByAddress[_id][msg.sender] -= _amount; // Would underflow if user did not have enough tokens
185	
186	        // Send the funds to the user
187	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim + price - fee);
188	        emit SharesSold(_id, msg.sender, _amount, price, fee);
189	    }
...
194	    function getNFTMintingPrice(uint256 _id, uint256 _amount) public view returns (uint256 fee) {
195	        address bondingCurve = shareData[_id].bondingCurve;
196	        (uint256 priceForOne, ) = IBondingCurve(bondingCurve).getPriceAndFee(shareData[_id].tokenCount, 1);
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
198	    }
...
226	    function burnNFT(uint256 _id, uint256 _amount) external {
227	        uint256 fee = getNFTMintingPrice(_id, _amount);
228	
229	        SafeERC20.safeTransferFrom(token, msg.sender, address(this), fee);
230	        _splitFees(_id, fee, shareData[_id].tokensInCirculation);
231	        // The user does not get the proportional rewards for the burning (unless they have additional tokens that are not in the NFT)
232	        uint256 rewardsSinceLastClaim = _getRewardsSinceLastClaim(_id);
233	        rewardsLastClaimedValue[_id][msg.sender] = shareData[_id].shareHolderRewardsPerTokenScaled;
234	        tokensByAddress[_id][msg.sender] += _amount;
235	        shareData[_id].tokensInCirculation += _amount;
236	        _burn(msg.sender, _id, _amount);
237	
238	        SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
239	        // ERC1155 already logs, but we add this to have the price information
240	        emit NFTsBurned(_id, msg.sender, _amount, fee);
241	    }
...
272	    function _getRewardsSinceLastClaim(uint256 _id) internal view returns (uint256 amount) {
273	        uint256 lastClaimedValue = rewardsLastClaimedValue[_id][msg.sender];
274	        amount =
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
276	            1e18;
277	    }
```
[132..136](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L132-L136)
[141..145](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L141-L145)
[174..189](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L174-L189)
[194..198](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L194-L198)
[226..241](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L226-L241)
[272..277](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L272-L277)

---

	 - asD/src/asD.sol

```solidity
47	    function mint(uint256 _amount) external {
48	        CErc20Interface cNoteToken = CErc20Interface(cNote);
49	        IERC20 note = IERC20(cNoteToken.underlying());
50	        SafeERC20.safeTransferFrom(note, msg.sender, address(this), _amount);
51	        SafeERC20.safeApprove(note, cNote, _amount);
52	        uint256 returnCode = cNoteToken.mint(_amount);
53	        // Mint returns 0 on success: https://docs.compound.finance/v2/ctokens/#mint
54	        require(returnCode == 0, "Error when minting");
55	        _mint(msg.sender, _amount);
56	    }
...
60	    function burn(uint256 _amount) external {
61	        CErc20Interface cNoteToken = CErc20Interface(cNote);
62	        IERC20 note = IERC20(cNoteToken.underlying());
63	        uint256 returnCode = cNoteToken.redeemUnderlying(_amount); // Request _amount of NOTE (the underlying of cNOTE)
64	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem-underlying
65	        _burn(msg.sender, _amount);
66	        SafeERC20.safeTransfer(note, msg.sender, _amount);
67	    }
...
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
73	        uint256 exchangeRate = CTokenInterface(cNote).exchangeRateCurrent(); // Scaled by 1 * 10^(18 - 8 + Underlying Token Decimals), i.e. 10^(28) in our case
74	        // The amount of cNOTE the contract has to hold (based on the current exchange rate which is always increasing) such that it is always possible to receive 1 NOTE when burning 1 asD
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
77	            totalSupply();
78	        if (_amount == 0) {
79	            _amount = maximumWithdrawable;
80	        } else {
81	            require(_amount <= maximumWithdrawable, "Too many tokens requested");
82	        }
83	        // Technically, _amount can still be 0 at this point, which would make the following two calls unnecessary.
84	        // But we do not handle this case specifically, as the only consequence is that the owner wastes a bit of gas when there is nothing to withdraw
85	        uint256 returnCode = CErc20Interface(cNote).redeemUnderlying(_amount);
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
87	        IERC20 note = IERC20(CErc20Interface(cNote).underlying());
88	        SafeERC20.safeTransfer(note, msg.sender, _amount);
89	        emit CarryWithdrawal(_amount);
90	    }
```
[47..56](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L47-L56)
[60..67](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L60-L67)
[72..90](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L72-L90)


</details>

-------
### [G-10] Optimize Zero Checks Using Assembly
<a name="G-10"></a>
[To the top](#TOP)

The usage of inline assembly to check if variable is the zero can save gas compared to traditional `require` or `if` statement checks. 

The assembly check uses the `extcodesize` operation which is generally cheaper in terms of gas.

[More information can be found here.](https://medium.com/@kalexotsu/solidity-assembly-checking-if-an-address-is-0-efficiently-d2bfe071331)

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 9 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
120	        require(shareIDs[_shareName] == 0, "Share already exists");
...
165	        if (rewardsSinceLastClaim > 0) {
...
216	        if (rewardsSinceLastClaim > 0) {
...
266	        if (amount > 0) {
...
289	        if (_tokenCount > 0) {
```
[120](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L120)
[165](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L165)
[216](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L216)
[266](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L266)
[289](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L289)

---

	 - asD/src/asD.sol

```solidity
54	        require(returnCode == 0, "Error when minting");
...
64	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem-underlying
...
78	        if (_amount == 0) {
...
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
```
[54](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L54)
[64](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L64)
[78](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L78)
[86](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L86)


</details>

-------
### [G-11] Add `unchecked` blocks for divisions where the operands cannot overflow
<a name="G-11"></a>
[To the top](#TOP)

`uint` divisions can't overflow, while `int` divisions can overflow only in [one specific case](https://docs.soliditylang.org/en/latest/types.html#division).

Consider adding an `unchecked` block to have some [gas savings](https://gist.github.com/DadeKuma/3bc597338ae774b8b3bd43280d55271f).

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 8 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
197	        fee = (priceForOne * _amount * NFT_FEE_BPS) / 10_000;
...
275	            ((shareData[_id].shareHolderRewardsPerTokenScaled - lastClaimedValue) * tokensByAddress[_id][msg.sender]) /
276	            1e18;
...
285	        uint256 shareHolderFee = (_fee * HOLDER_CUT_BPS) / 10_000;
...
286	        uint256 shareCreatorFee = (_fee * CREATOR_CUT_BPS) / 10_000;
...
290	            shareData[_id].shareHolderRewardsPerTokenScaled += (shareHolderFee * 1e18) / _tokenCount;
```
[197](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L197)
[275..276](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L275-L276)
[285](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L285)
[286](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L286)
[290](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L290)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
23	            fee += (getFee(i) * tokenPrice) / 1e18;
...
35	        return 1e17 / divisor;
```
[23](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L23)
[35](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L35)

---

	 - asD/src/asD.sol

```solidity
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
```
[75..76](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L75-L76)


</details>

-------
### [G-12] Trade-offs Between Modifiers and Internal Functions
<a name="G-12"></a>
[To the top](#TOP)

In Solidity, both internal functions and modifiers are used to refactor and manage code, but they come with their own trade-offs, especially in terms of gas cost and flexibility.

#### Modifiers:
- Less runtime gas cost (saves around 24 gas per function call).
- Increases deployment gas cost due to repetitive code.
- Can only be executed at the start or end of a function.

#### Internal Functions:
- Lower deployment cost.
- Can be executed at any point in a function.
- Slightly higher runtime gas cost (24 gas) due to the need to jump to the function's location in bytecode.

#### Recommendations:
- Use modifiers for high-frequency functions where runtime gas cost matters the most.
- Use internal functions where the priority is reducing deployment gas cost or when you need more flexibility in the function's logic.

Example analysis shows that using modifiers can increase deployment costs by over 35k gas but save 24 gas per function call during runtime. Choose wisely based on your specific use case.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 7 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
80	    modifier onlyShareCreator() {
81	        require(
82	            !shareCreationRestricted || whitelistedShareCreators[msg.sender] || msg.sender == owner(),
83	            "Not allowed"
84	        );
85	        _;
86	    }
...
104	    function changeBondingCurveAllowed(address _bondingCurve, bool _newState) external onlyOwner {
...
118	    ) external onlyShareCreator returns (uint256 id) {
...
244	    function claimPlatformFee() external onlyOwner {
...
300	    function restrictShareCreation(bool _isRestricted) external onlyOwner {
...
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
```
[80..86](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L80-L86)
[104](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L104)
[118](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L118)
[244](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L244)
[300](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L300)
[309](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L309)

---

	 - asD/src/asD.sol

```solidity
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
```
[72](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L72)


</details>

-------
### [G-13] Multiple mappings can be replaced with a single struct mapping
<a name="G-13"></a>
[To the top](#TOP)

Saves a storage slot for the mapping. Depending on the circumstances and sizes of types, can avoid a Gsset (20000 gas) per mapping combined. Reads and subsequent writes can also be cheaper when a function requires both values and they both fit in the same storage slot.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 6 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
49	    mapping(address => bool) public whitelistedBondingCurves;
...
64	    mapping(address => bool) public whitelistedShareCreators;
...
43	    mapping(uint256 => ShareData) public shareData;
...
46	    mapping(uint256 => address) public shareBondingCurves;
...
52	    mapping(uint256 => mapping(address => uint256)) public tokensByAddress;
...
55	    mapping(uint256 => mapping(address => uint256)) public rewardsLastClaimedValue;
```
[49](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L49)
[64](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L64)
[43](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L43)
[46](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L46)
[52](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L52)
[55](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L55)


</details>

-------
### [G-14] Functions guaranteed to revert when called by normal users can be marked `payable`
<a name="G-14"></a>
[To the top](#TOP)

If a function modifier such as `onlyOwner` is used, the function will revert if a normal user tries to pay the function. Marking the function as `payable` will lower the gas cost for legitimate callers because the compiler will not include checks for whether a payment was provided. The extra opcodes avoided are `CALLVALUE`(2),`DUP1`(3),`ISZERO`(3),`PUSH2`(3),`JUMPI`(10),`PUSH1`(3),`DUP1`(3),`REVERT`(0),`JUMPDEST`(1),`POP`(2), which costs an average of about 21 gas per call to the function, in addition to the extra deployment cost


#### <ins>Proof Of Concept</ins>

<details>

<summary>see 5 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
104	    function changeBondingCurveAllowed(address _bondingCurve, bool _newState) external onlyOwner {
105	        require(whitelistedBondingCurves[_bondingCurve] != _newState, "State already set");
106	        whitelistedBondingCurves[_bondingCurve] = _newState;
107	        emit BondingCurveStateChange(_bondingCurve, _newState);
108	    }
...
244	    function claimPlatformFee() external onlyOwner {
245	        uint256 amount = platformPool;
246	        platformPool = 0;
247	        SafeERC20.safeTransfer(token, msg.sender, amount);
248	        emit PlatformFeeClaimed(msg.sender, amount);
249	    }
...
300	    function restrictShareCreation(bool _isRestricted) external onlyOwner {
301	        require(shareCreationRestricted != _isRestricted, "State already set");
302	        shareCreationRestricted = _isRestricted;
303	        emit ShareCreationRestricted(_isRestricted);
304	    }
...
309	    function changeShareCreatorWhitelist(address _address, bool _isWhitelisted) external onlyOwner {
310	        require(whitelistedShareCreators[_address] != _isWhitelisted, "State already set");
311	        whitelistedShareCreators[_address] = _isWhitelisted;
312	    }
```
[104..108](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L104-L108)
[244..249](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L244-L249)
[300..304](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L300-L304)
[309..312](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L309-L312)

---

	 - asD/src/asD.sol

```solidity
72	    function withdrawCarry(uint256 _amount) external onlyOwner {
73	        uint256 exchangeRate = CTokenInterface(cNote).exchangeRateCurrent(); // Scaled by 1 * 10^(18 - 8 + Underlying Token Decimals), i.e. 10^(28) in our case
74	        // The amount of cNOTE the contract has to hold (based on the current exchange rate which is always increasing) such that it is always possible to receive 1 NOTE when burning 1 asD
75	        uint256 maximumWithdrawable = (CTokenInterface(cNote).balanceOf(address(this)) * exchangeRate) /
76	            1e28 -
77	            totalSupply();
78	        if (_amount == 0) {
79	            _amount = maximumWithdrawable;
80	        } else {
81	            require(_amount <= maximumWithdrawable, "Too many tokens requested");
82	        }
83	        // Technically, _amount can still be 0 at this point, which would make the following two calls unnecessary.
84	        // But we do not handle this case specifically, as the only consequence is that the owner wastes a bit of gas when there is nothing to withdraw
85	        uint256 returnCode = CErc20Interface(cNote).redeemUnderlying(_amount);
86	        require(returnCode == 0, "Error when redeeming"); // 0 on success: https://docs.compound.finance/v2/ctokens/#redeem
87	        IERC20 note = IERC20(CErc20Interface(cNote).underlying());
88	        SafeERC20.safeTransfer(note, msg.sender, _amount);
89	        emit CarryWithdrawal(_amount);
90	    }
```
[72..90](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L72-L90)


</details>

-------
### [G-15] `>=` costs less gas than `>`
<a name="G-15"></a>
[To the top](#TOP)

The compiler uses opcodes `GT` and `ISZERO` for solidity code that uses `>`, but only requires LT for `>=`, which saves 3 gas. Similarly for `<=`.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 5 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
165	        if (rewardsSinceLastClaim > 0) {
...
216	        if (rewardsSinceLastClaim > 0) {
...
266	        if (amount > 0) {
...
289	        if (_tokenCount > 0) {
```
[165](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L165)
[216](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L216)
[266](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L266)
[289](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L289)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
29	        if (shareCount > 1) {
```
[29](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L29)


</details>

-------
### [G-16] Setting the `constructor` to `payable`
<a name="G-16"></a>
[To the top](#TOP)

Saves ~13 gas per instance

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
91	    constructor(string memory _uri, address _paymentToken) ERC1155(_uri) Ownable() {
```
[91](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L91)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
10	    constructor(uint256 _priceIncrease) {
```
[10](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L10)

---

	 - asD/src/asD.sol

```solidity
28	    constructor(
29	        string memory _name,
30	        string memory _symbol,
31	        address _owner,
32	        address _cNote,
33	        address _csrRecipient
34	    ) ERC20(_name, _symbol) {
```
[28..34](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L28-L34)

---

	 - asD/src/asDFactory.sol

```solidity
24	    constructor(address _cNote) {
```
[24](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L24)


</details>

-------
### [G-17] Using `bool` for storage incurs overhead
<a name="G-17"></a>
[To the top](#TOP)

Use `uint256(1)` and `uint256(2)` for `true`/`false` to avoid a Gwarmaccess (100 gas), and to avoid Gsset (20000 gas) when changing from `false` to `true`, after having been `true` in the past. See [source](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/58f635312aa21f947cae5f8578638a85aa2519f5/contracts/security/ReentrancyGuard.sol#L23-L27).

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
49	    mapping(address => bool) public whitelistedBondingCurves;
...
61	    bool public shareCreationRestricted = true;
...
64	    mapping(address => bool) public whitelistedShareCreators;
```
[49](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L49)
[61](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L61)
[64](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L64)

---

	 - asD/src/asDFactory.sol

```solidity
15	    mapping(address => bool) public isAsD;
```
[15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L15)


</details>

-------
### [G-18] Use at least Solidity version `0.8.19` to gain some gas boost
<a name="G-18"></a>
[To the top](#TOP)

Upgrade to at least solidity version 0.8.19 to get additional gas savings. Check the [documentation](https://blog.soliditylang.org/2263/26/22/solidity-0.8.19-release-announcement/) for reference.

Some additional details:
> In earlier releases and in the default legacy code generation, when an internal library function or a free function accessed via a module was called only during contract creation, e.g. only in the constructor, a copy of the function still also occurred in the contract’s runtime bytecode.
>
>So a function pointer in creation code also refers to the offset of the function in runtime code, which requires the function to actually be present in runtime code.
>
>For direct calls to internal contract functions the full encoding of the function expression is bypassed by the compiler. However, this bypassing did not happen for internal library functions and for free functions called via modules, causing the undesirable behaviour that is now fixed in this release.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
2	pragma solidity 0.8.19;
```
[2](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L2)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
2	pragma solidity 0.8.19;
```
[2](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L2)

---

	 - asD/src/asD.sol

```solidity
2	pragma solidity >=0.8.0;
```
[2](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L2)

---

	 - asD/src/asDFactory.sol

```solidity
2	pragma solidity >=0.8.0;
```
[2](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L2)


</details>

-------
### [G-19] Optimize Boolean States with `uint256(1/2)`
<a name="G-19"></a>
[To the top](#TOP)

Boolean variables in Solidity are more expensive than `uint256` or any type that takes up a full word, due to additional gas costs associated with write operations.
When using boolean variables, each write operation emits an extra SLOAD to read the slot's contents, replace the bits taken up by the boolean, and then write back.
This process cannot be disabled and leads to extra gas consumption.

By using `uint256(1)` and `uint256(2)` for representing true and false states, you can avoid a `Gwarmaccess` (100 gas) cost and also avoid a `Gsset` (20000 gas) cost when changing from `false` to `true`, after having been `true` in the past.
This approach helps in optimizing gas usage, making your contract more cost-effective.

[Usage in OpenZeppelin ReentrancyGuard.sol](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/58f635312aa21f947cae5f8578638a85aa2519f5/contracts/security/ReentrancyGuard.sol#L23-L27)

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
49	    mapping(address => bool) public whitelistedBondingCurves;
...
61	    bool public shareCreationRestricted = true;
...
64	    mapping(address => bool) public whitelistedShareCreators;
```
[49](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L49)
[61](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L61)
[64](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L64)

---

	 - asD/src/asDFactory.sol

```solidity
15	    mapping(address => bool) public isAsD;
```
[15](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L15)


</details>

-------
### [G-20] Optimize Unsigned Integer Comparison With Zero
<a name="G-20"></a>
[To the top](#TOP)

For unsigned integers, checking whether the integer is not equal to zero (`!= 0`) is less gas-intensive than checking whether it is greater than zero (`> 0`). 

This is because the Ethereum Virtual Machine (EVM) can perform a simple bitwise operation to check if any bit is set (which directly translates to `!= 0`), while checking for `> 0` requires additional logic.

As such, when dealing with unsigned integers in Solidity, it is recommended to use the `!= 0` comparison for gas optimization.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 4 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
165	        if (rewardsSinceLastClaim > 0) {
...
216	        if (rewardsSinceLastClaim > 0) {
...
266	        if (amount > 0) {
...
289	        if (_tokenCount > 0) {
```
[165](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L165)
[216](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L216)
[266](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L266)
[289](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L289)


</details>

-------
### [G-21] Using `private` for constants saves gas
<a name="G-21"></a>
[To the top](#TOP)

Saves deployment gas due to the compiler not having to create non-payable getter functions for deployment calldata, not having to store the bytes of the value outside of where it's used, and not adding another entry to the method ID table.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 3 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
...
15	    uint256 public constant HOLDER_CUT_BPS = 3_300; // 33%
...
16	    uint256 public constant CREATOR_CUT_BPS = 3_300; // 33%
```
[14](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L14)
[15](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L15)
[16](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L16)


</details>

-------
### [G-22] Using `private` for `constants` saves gas
<a name="G-22"></a>
[To the top](#TOP)

Saves deployment gas due to the compiler not having to create non-payable getter functions for deployment calldata, not having to store the bytes of the value outside of where it's used, and not adding another entry to the method ID table.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 3 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
14	    uint256 public constant NFT_FEE_BPS = 1_000; // 10%
...
15	    uint256 public constant HOLDER_CUT_BPS = 3_300; // 33%
...
16	    uint256 public constant CREATOR_CUT_BPS = 3_300; // 33%
```
[14](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L14)
[15](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L15)
[16](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L16)


</details>

-------
### [G-23] Delete Unused State Variables
<a name="G-23"></a>
[To the top](#TOP)

State variables that aren't used in the contract not only clutter the codebase but also consume unnecessary gas during deployment.
Specifically, setting non-zero initial values for state variables costs significant gas.
By removing these unused state variables, you can save on both deployment gas and potential future storage gas costs.
This optimization not only reduces gas expenditures but also enhances code clarity and maintainability.
Always ensure a thorough review to confirm that these variables are indeed redundant before removal.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 3 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
46	    mapping(uint256 => address) public shareBondingCurves;
```
[46](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L46)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
42	    function log2(uint256 x) internal pure returns (uint256 r) {
...
42	    function log2(uint256 x) internal pure returns (uint256 r) {
```
[42](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L42)
[42](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L42)


</details>

-------
### [G-24] Optimize Gas Spend Using `0.8.20` and Optimizer Features
<a name="G-24"></a>
[To the top](#TOP)

New features introduced in Solidity 0.8.20 that enhance gas efficiency.
Specifically, it takes advantage of the `push0` assembler operation for placing 0 on the EVM stack, which reduces both deployment and runtime costs.

Furthermore, it utilizes the re-implemented versions of the `UnusedAssignEliminator` and `UnusedStoreEliminator` in the Solidity optimizer, eliminating unused assignments in deeply nested loops and thus further reducing the gas required for contract execution.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
2	pragma solidity 0.8.19;
```
[2](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L2)

---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
2	pragma solidity 0.8.19;
```
[2](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L2)


</details>

-------
### [G-25] Optimize Gas by Using Only Named Returns
<a name="G-25"></a>
[To the top](#TOP)

The Solidity compiler can generate more efficient bytecode when using named returns.
It's recommended to replace anonymous returns with named returns for potential gas savings.

Example:
```solidity
/// 985 gas cost
function add(uint256 x, uint256 y) public pure returns (uint256) {
  return x + y;
}
/// 941 gas cost
function addNamed(uint256 x, uint256 y) public pure returns (uint256 res) {
  res = x + y;
}
```

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
27	    function getFee(uint256 shareCount) public pure override returns (uint256) {
28	        uint256 divisor;
29	        if (shareCount > 1) {
30	            divisor = log2(shareCount);
31	        } else {
32	            divisor = 1;
33	        }
34	        // 0.1 / log2(shareCount)
35	        return 1e17 / divisor;
36	    }
```
[27..36](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L27-L36)

---

	 - asD/src/asDFactory.sol

```solidity
33	    function create(string memory _name, string memory _symbol) external returns (address) {
34	        asD createdToken = new asD(_name, _symbol, msg.sender, cNote, owner());
35	        isAsD[address(createdToken)] = true;
36	        emit CreatedToken(address(createdToken), _symbol, _name, msg.sender);
37	        return address(createdToken);
38	    }
```
[33..38](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L33-L38)


</details>

-------
### [G-26] `++i`/`i++` should be `unchecked{++i}`/`unchecked{i++}` when it is not possible for them to overflow, as is the case when used in `for`- and `while`-loops
<a name="G-26"></a>
[To the top](#TOP)

The `unchecked` keyword is new in solidity version 0.8.0, so this only applies to that version or higher, which these instances are. This saves 30-40 gas [per loop](https://gist.github.com/hrkrshnn/ee8fabd532058307229d65dcd5836ddc#the-increment-in-for-loop-post-condition-can-be-made-unchecked)

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
20	        for (uint256 i = shareCount; i < shareCount + amount; i++) {
21	            uint256 tokenPrice = priceIncrease * i;
22	            price += tokenPrice;
23	            fee += (getFee(i) * tokenPrice) / 1e18;
24	        }
```
[20..24](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L20-L24)


</details>

-------
### [G-27] Pre-increments and pre-decrements are cheaper than post-increments and post-decrements
<a name="G-27"></a>
[To the top](#TOP)

*Saves 5 gas per iteration*

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
20	        for (uint256 i = shareCount; i < shareCount + amount; i++) {
```
[20](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L20)


</details>

-------
### [G-28] `Internal` functions only called once can be inlined to save gas
<a name="G-28"></a>
[To the top](#TOP)

---

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
42	    function log2(uint256 x) internal pure returns (uint256 r) {
43	        /// @solidity memory-safe-assembly
44	        assembly {
45	            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
46	            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
47	            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
48	            r := or(r, shl(4, lt(0xffff, shr(r, x))))
49	            r := or(r, shl(3, lt(0xff, shr(r, x))))
50	            // forgefmt: disable-next-item
51	            r := or(
52	                r,
53	                byte(
54	                    and(0x1f, shr(shr(r, x), 0x8421084210842108cc6318c6db6d54be)),
55	                    0x0706060506020504060203020504030106050205030304010505030400000000
56	                )
57	            )
58	        }
59	    }
```
[42..59](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L42-L59)


</details>

-------
### [G-29] State variables access within a loop
<a name="G-29"></a>
[To the top](#TOP)

State variable reads and writes are more expensive than local variable reads and writes. Therefore, it is recommended to replace state variable reads and writes within loops with a local variable. Gas savings should be multiplied by the average loop length.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
20	        for (uint256 i = shareCount; i < shareCount + amount; i++) {
21	            uint256 tokenPrice = priceIncrease * i;
22	            price += tokenPrice;
23	            fee += (getFee(i) * tokenPrice) / 1e18;
24	        }
```
[20..24](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L20-L24)


</details>

-------
### [G-30] Consider activating `via-ir` for deploying
<a name="G-30"></a>
[To the top](#TOP)

The IR-based code generator was developed to make code generation more performant by enabling optimization passes that can be applied across functions.

It is possible to activate the IR-based code generator through the command line by using the flag `--via-ir` or by including the option `{"viaIR": true}`.

Keep in mind that compiling with this option may take longer. However, you can simply test it before deploying your code. If you find that it provides better performance, you can add the `--via-ir` flag to your deploy command.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>

Placeholder, for non-solidity issues.


[C:/Users/pasha/code/arena4bot/project/2023-11-canto](https://github.com/code-423n4/2023-11-canto/blob/main/)


</details>

-------
### [G-31] Optimize Gas by Using Do-While Loops
<a name="G-31"></a>
[To the top](#TOP)

Using `do-while` loops instead of `for` loops can be more gas-efficient. 
Even if you add an `if` condition to account for the case where the loop doesn't execute at all, a `do-while` loop can still be cheaper in terms of gas.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 1 instances</summary>


---

	 - 1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol

```solidity
20	        for (uint256 i = shareCount; i < shareCount + amount; i++) {
21	            uint256 tokenPrice = priceIncrease * i;
22	            price += tokenPrice;
23	            fee += (getFee(i) * tokenPrice) / 1e18;
24	        }
```
[20..24](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/bonding_curve/LinearBondingCurve.sol#L20-L24)


</details>


## Disputed Risk Issues

### [D-01] Control structures do not comply with best practices
<a name="D-01"></a>
[To the top](#TOP)

Usually, most instances are comply with best practices, it just report when true/false body is not block, but it is allowed by style guade when statement is one line.

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 6 instances</summary>


---

	 - 1155tech-contracts/src/Market.sol

```solidity
93	        if (block.chainid == 7700 || block.chainid == 7701) {
94	            // Register CSR on Canto main- and testnet
95	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
96	            turnstile.register(tx.origin);
97	        }
...
165	        if (rewardsSinceLastClaim > 0) {
166	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
167	        }
...
216	        if (rewardsSinceLastClaim > 0) {
217	            SafeERC20.safeTransfer(token, msg.sender, rewardsSinceLastClaim);
218	        }
...
266	        if (amount > 0) {
267	            SafeERC20.safeTransfer(token, msg.sender, amount);
268	        }
```
[93..97](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L93-L97)
[165..167](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L165-L167)
[216..218](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L216-L218)
[266..268](https://github.com/code-423n4/2023-11-canto/blob/main/1155tech-contracts/src/Market.sol#L266-L268)

---

	 - asD/src/asD.sol

```solidity
37	        if (block.chainid == 7700 || block.chainid == 7701) {
38	            // Register CSR on Canto main- and testnet
39	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
40	            turnstile.register(_csrRecipient);
41	        }
```
[37..41](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L37-L41)

---

	 - asD/src/asDFactory.sol

```solidity
26	        if (block.chainid == 7700 || block.chainid == 7701) {
27	            // Register CSR on Canto main- and testnet
28	            Turnstile turnstile = Turnstile(0xEcf044C5B4b867CFda001101c617eCd347095B44);
29	            turnstile.register(tx.origin);
30	        }
```
[26..30](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L26-L30)


</details>

-------
### [D-02] Optimize Gas Spend Using `0.8.20` and Optimizer Features
<a name="D-02"></a>
[To the top](#TOP)

The rule itself is correct. But here are the cases in which SemVer allows you to use version up to `0.8.20`

#### <ins>Proof Of Concept</ins>

<details>

<summary>see 2 instances</summary>


---

	 - asD/src/asD.sol

```solidity
2	pragma solidity >=0.8.0;
```
[2](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asD.sol#L2)

---

	 - asD/src/asDFactory.sol

```solidity
2	pragma solidity >=0.8.0;
```
[2](https://github.com/code-423n4/2023-11-canto/blob/main/asD/src/asDFactory.sol#L2)


</details>

