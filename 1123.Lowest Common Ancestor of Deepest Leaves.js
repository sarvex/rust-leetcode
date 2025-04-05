/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */

/**
 * Finds LCA of deepest leaves using DFS with global maxDepth tracking
 * 
 * #intuition
 * - Track global maxDepth to identify deepest level
 * - Node is LCA when both subtrees reach maxDepth
 * - Update LCA when finding equal depths at maxDepth
 * 
 * #approach
 * 1. Use DFS with depth parameter to track current depth
 * 2. Update maxDepth when reaching deeper nodes
 * 3. Compare left and right subtree depths
 * 4. Set node as LCA when both subtrees reach maxDepth
 * 5. Return maximum depth of current subtree
 * 
 * #complexity
 * Time: O(n) where n is number of nodes, single DFS pass
 * Space: O(h) where h is tree height, recursion stack only
 * 
 * @param {TreeNode} root - Root node of the binary tree
 * @return {TreeNode} - Lowest common ancestor of deepest leaves
 */
var lcaDeepestLeaves = function(root) {
    let maxDepth = -1;
    let lca = null;
    
    function dfs(node, depth) {
        if (!node) {
            return depth - 1;
        }
        
        // Update maxDepth when reaching deeper level
        if (depth > maxDepth) {
            maxDepth = depth;
        }
        
        const leftDepth = dfs(node.left, depth + 1);
        const rightDepth = dfs(node.right, depth + 1);
        
        // Update LCA when both subtrees reach maxDepth
        if (leftDepth === rightDepth && leftDepth === maxDepth) {
            lca = node;
        }
        
        return Math.max(leftDepth, rightDepth);
    }
    
    dfs(root, 0);
    return lca;
};
