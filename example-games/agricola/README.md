## Agricola Implementation

This implementation is for the family game (aka no minor improvements or occupations) for 2 players.

### Assumptions made

This implementation revolves around only one action being passed back to the UCF function. Since there are many sub-actions in agricola (fence placement and animal placement for example), these actions are randomized and then proceeded forward. In theory, for the subactions to be propagated through the game state tree, there would probably need to be a tuple of actions, where the subactions are passed alongside the main action.

* When a fences action is taken, a random number of pastures are placed 50% of the time until wood is depleted (player.rs:343)
* Animal placement is randomized 20 times and the placement with the best score is continued forward and all other animals are killed (player.rs/382)
* Each time animals are placed, the entire animal setup is reset, in an attempt to find the best solution
* Animals are placed in pastures > stables > house order
* Stables are built randomly and not necessarily the most stables for the available wood
* If food is still needed in feeding and the player has a fireplace and/or cooking hearth, there is a random draw to determine what animal to kill. This decision is based on which would cause the best board score
* If MajorImprovement is taken for Cooking Hearth, and there is an option for paying via clay or exchanging a fireplace, there is a 50% chance of exchanging vs paying for the improvement outright.
* Baking Bread will cook all available grain if possible
* Pottery/Joinery/Basketmaker's Workshop will always be used if possible during harvest
