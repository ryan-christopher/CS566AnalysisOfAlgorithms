# Define the graph as an adjacency list
graph = {
    'A': ['B', 'C'],
    'B': ['D'],
    'C': [],
    'D': [],
    'E': ['B'],
}

graph2 = {
    'A': ['B', 'C', 'F'],
    'B': ['C', 'D'],
    'C': [],
    'D': ['A', 'C'],
    'E': ['C', 'G'],
    'F': ['A', 'C'],
    'G': ['D', 'E']
}

# Helper function to perform DFS
def dfs(graph, node, visited, tree):
    visited.add(node)
    for neighbor in graph[node]:
        if neighbor not in visited:
            tree[node].append(neighbor)
            dfs(graph, neighbor, visited, tree)

# Function to create a DFS tree
def dfs_tree(graph, start):
    visited = set()
    tree = {node: [] for node in graph}  # Create a tree structure with empty lists
    dfs(graph, start, visited, tree)
    return tree

# Create DFS tree starting from node 'A'
# start_node = 'G'
# tree = dfs_tree(graph2, start_node)
# for item in tree:
#     print(item, tree[item])

import heapq

def prim(graph, start_vertex):
    mst = []  # List to store the MST edges
    visited = set()  # Set to track visited vertices
    min_heap = [(0, start_vertex)]  # Min-heap to get the edge with the smallest weight
    
    while min_heap:
        weight, vertex = heapq.heappop(min_heap)
        
        if vertex in visited:
            continue
        
        visited.add(vertex)
        mst.append((weight, vertex))
        
        for neighbor, edge_weight in graph[vertex]:
            if neighbor not in visited:
                heapq.heappush(min_heap, (edge_weight, neighbor))
    
    return mst

graph3 = {
    'A': [('B', 2),('F', 7),('G', 3)],
    'B': [('A', 2),('C', 4),('G', 6)],
    'C': [('B', 4),('D', 2),('H', 2)],
    'D': [('C', 2),('E', 1),('H', 8)],
    'E': [('D', 1),('F', 6),('I', 2)],
    'F': [('A', 7),('E', 6),('I', 5)],
    'G': [('A', 3),('B', 6),('H', 3),('I', 1)],
    'H': [('C', 2),('D', 8),('G', 3),('I', 4)],
    'I': [('E', 2),('F', 5),('G', 1),('H', 4)],
}

#print(prim(graph3, 'H'))

def dijkstra(graph, start):
    distances = {node: float('inf') for node in graph}
    distances[start] = 0
    priority_queue = [(0, start)]  
    previous_nodes = {node: None for node in graph}  
    
    while priority_queue:
        current_distance, current_node = heapq.heappop(priority_queue)
        
        if current_distance > distances[current_node]:
            continue
        
        for neighbor, weight in graph[current_node]:
            distance = current_distance + weight
            if distance < distances[neighbor]:
                distances[neighbor] = distance
                previous_nodes[neighbor] = current_node
                heapq.heappush(priority_queue, (distance, neighbor))
    
    return distances, previous_nodes

# The given graph
graph4 = {
    'A': [('B', 4.0), ('F', 2.0)],
    'B': [('A', 1.0), ('C', 3.0), ('D', 4.0)],
    'C': [('A', 6.0), ('B', 3.0), ('D', 7.0)],
    'D': [('A', 6.0), ('E', 2.0)],
    'E': [('D', 5.0)],
    'F': [('D', 2.0), ('E', 3.0)]
}

# Execute Dijkstra's algorithm starting from node 'A'
distances, previous_nodes = dijkstra(graph4, 'A')

print("Distances from A:", distances)
print("Previous nodes:", previous_nodes)