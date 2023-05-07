# DS210-Final-Project
Analyzing the Gender Distribution of Facebook User’s Connections

**Project Goal and Description**

I created this project because I am interested in strategy and analytics, and this project analyzes data in order to get the most efficient and impactful marketing strategy. The dataset for this project comes from https://snap.stanford.edu/data/ego-Facebook.html and has an undirected graph of 4039 individuals connected to one another. The dataset also had anonymized characteristics for each of these 4039 people, and I thought it would be interesting to see how many of each gender each person was connected to. This would be helpful for any company in need of ambassadors to market a specific product. For example, a company selling feminine products would want to find someone with not just the most followers to promote their product but whoever had a higher concentration of female followers. So someone with 1000 followers but only 30% are female is a worse candidate than someone with 600 followers but 60% are female. The dataset doesn’t directly say male or female, just: gender; anonymized feature 78 and 79, but I am going in with the assumption that a company using this software would know which one matched male and which one was for females.

There are three main functions in this project that would be helpful to a client. The first is eligible_nodes, which takes in a graphs adjacency list, and a list of nodes and whether they have the gender feature or not(0 or 1, the minimum connections required, and the minimum percentage of the select gender feature in the followers, and returns a vector of tuples that satisfy the eligibility in the form [node id, connections, fraction of the select gender feature in degree, number of connections that posses the gender feature]. Then the functions highest_dummy and lowest_dummy will rank the tuples based on the highest or lowest amount of connections with the gender variable you’re looking for. Highest_dummy is helpful when you are looking for someone with a lot of feature 78 and lowest_dummy is when you’re looking for someone with a lot of feature 77 followers. These two functions are essential when looking for an ambassador for a gender-specific product.

Additionally, The functions, highest_pct and highest_degree, then take those eligible vectors and sort them in descending order by highest percentage concentration and highest degree respectively if that is what you are curious about.


**How to Run the Code**

Firstly, this link does not include the data files, so you will have to download the data yourself using the link above and place it in a folder with my source and cargo code. In the folder with my code make sure to have a folder called facebook which contains the all the ego-nodes and their features. Also in the folder with my code be sure to have the faceboo_edges.txt file. On main.rs, you can control the degree and pct(percentage) arguments to use the functions eligible_nodes, highest_dummy, lowest_dummy, highest_pct, and highest_degree to your advantage.

In my source file, you will also find the modules graph.rs, filters.rs, features.rs, and test.rs. They all have small comments explaining their purpose and how they relate to the main function. Specifically, test.rs contains sample data that runs through the same code in main.rs, and checks to see if my functions work(they do!). 

To run the code, you can just type in cargo run into your terminal and that should run all the necessary code in main.rs.

**Output**

Once the main function runs it will output 5 things for the functions: eligible_nodes, highest_dummy, lowest_dummy, highest_pct, and highest_degree

Here is a sample output with the minimum degree parameter as 29 and the minimum percentage as 0.6:

The nodes that have a minimum degree of 29 and a minimum percenatge of 0.6 of gender;anonymized 1:
 [(2257, 38, 0.6052632, 23), (2547, 39, 0.61538464, 24), (3224, 51, 0.60784316, 31), (3327, 33, 0.6060606, 20)]
The nodes that have a minimum degree of 29 and a minimum percenatge of 0.6 of gender;anonymized 1 ordered by DEGREE:
[(3224, 51, 0.60784316, 31), (2547, 39, 0.61538464, 24), (2257, 38, 0.6052632, 23), (3327, 33, 0.6060606, 20)]
The nodes that have a minimum degree of 29 and a minimum percenatge of 0.6 of gender;anonymized 1 ordered by PERCENTAGE:
[(2547, 39, 0.61538464, 24), (3224, 51, 0.60784316, 31), (3327, 33, 0.6060606, 20), (2257, 38, 0.6052632, 23)]
The nodes that have a minimum degree of 29 and a minimum percenatge of 0.6 of gender;anonymized 1 ordered by HIGHEST DUMMEY:
[(3224, 51, 0.60784316, 31), (2547, 39, 0.61538464, 24), (2257, 38, 0.6052632, 23), (3327, 33, 0.6060606, 20)]
The nodes that have a minimum degree of 29 and a minimum percenatge of 0.6 of gender;anonymized 1 ordered by LOWEST DUMMEY:
[(3327, 33, 0.6060606, 20), (2257, 38, 0.6052632, 23), (2547, 39, 0.61538464, 24), (3224, 51, 0.60784316, 31)]

Just to reiterate, each tuple follows the structure: (node id, connections, proportion of gender variable 78 = 1, connections where gender variable 78 = 1)

**Sources**

https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html

Also, the TA Alicja helped me debug and brainstorm project ideas.
