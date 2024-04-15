/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

//I AM NOT 
use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph 无向图的节点  这里实际上是一个邻接表
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-firste sarch on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
		//TODO
        // 新增一个遍历队列
        let mut taskQueue = VecDeque::new();
        // 结果数组
        let mut visit_order = vec![];
        // usize具有copy trait
        // visit_order.push(start);  // 将开始节点编号加入
        taskQueue.push_back(start);// 将开始节点编号加入队列
        // 开始bfs
        loop {
            let items = taskQueue.pop_front();// 找到队首元素编号
            if let Some(item) = items {
                if !visit_order.contains(&item) {
                    visit_order.push(item); // 将节点编号加入
                }
                // 获取该节点的邻居
                let near_element:Vec<usize> = self.adj[item].clone().iter()
                                    .filter(|t| !(&visit_order).contains(t))
                                    .map(|element| (*element).clone())
                                    .collect();
                // 将邻居加入双端队列
                for i in near_element {
                    taskQueue.push_back(i.clone())
                }
            }else {
                break; // pop不出来有用的东西
            }
        }

        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

