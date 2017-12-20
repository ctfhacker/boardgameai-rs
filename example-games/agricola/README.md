## Agricola Implementation

### Assumptions made

This implementation revolves around only one action being passed back to the UCF function. Since there are many sub-actions in agricola (fence placement and animal placement for example), these actions are randomized and then proceeded forward. In theory, for the subactions to be propagated through the game state tree, there would probably need to be a tuple of actions, where the subactions are passed alongside the main action.

* When a fences action is taken, a random number of pastures are placed 50% of the time until wood is depleted (player.rs:343)
* Animal placement is randomized 20 times and the placement with the best score is continued forward and all other animals are killed (player.rs/382)
* Each time animals are placed, the entire animal setup is reset, in an attempt to find the best solution
* Animals are placed in pastures > stables > house order
* Stables are built randomly and not necessarily the most stables for the available wood
