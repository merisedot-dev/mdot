Feature: Writing down SQL scripts.

    Scenario Outline: Empty database
        Given a new graph
        And there are 0 entities in graph
        And we want to name the database "<name>"
        When we convert the graph using the <kernel> conversion core
        Then the resulting script looks like `assets/empty/<name>.sql`

        Examples:
            |name|kernel|
            |msql|MySql |

    Scenario Outline: Only entities
        Given a new graph
        And there are <nbe> entities in graph
        And each entity has a primary key
        And we want to name the database "<name>"
        When we convert the graph using the <kernel> conversion core
        Then the resulting script looks like `assets/oe/<name>.sql`

        Examples:
            |nbe|name  |kernel|
            |4  |msql01|MySql |

    Scenario Outline: Entities and links
        Given a new graph
        And there are <nbe> entities in graph
        And each entity has a primary key
        And entities <n1> and <n2> are linked via "l1"
        And ent<n1>'s cardinalities are <m1>,<M1>
        And ent<n2>'s cardinalities are <m2>,<M2>
        And we want to name the database "<name>"
        When we convert the graph using the <kernel> conversion core
        Then the resulting script looks like `assets/el/<name>.sql`

        Examples:
            |nbe|n1|m1|M1|n2|m2|M2|name  |kernel|
            |4  |1 |0 |n |2 |0 |n |msql01|MySql |
            |3  |1 |1 |1 |3 |0 |n |msql02|MySql |

    Scenario Outline: Full database
        Given a new graph
        And there are 5 entities in graph
        And each entity has a primary key
        And entities 1 and 3 are linked via "l1"
        And ent1's cardinalities are 0,n
        And ent3's cardinalities are 0,n
        And entities 1 and 4 are linked via "l1"
        And ent1's cardinalities are 0,n
        And ent4's cardinalities are 0,n
        And entities 2 and 4 are linked via "l2"
        And ent2's cardinalities are 0,n
        And ent4's cardinalities are 0,n
        And entities 3 and 5 are linked via "l3"
        And ent3's cardinalities are 1,1
        And ent5's cardinalities are 0,1
        And we want to name the database "<name>"
        When we convert the graph using the <kernel> conversion core
        Then the resulting script looks like `assets/db/<name>.sql`

        Examples:
            |name  |kernel|
            |msql01|MySql |
