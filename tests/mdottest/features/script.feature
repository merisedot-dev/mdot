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
        When we convert the graph using the <kernel> conversion core
        Then the resulting script looks like `assets/oe/<name>.sql`

        Examples:
            |nbe|name  |kernel|
            |4  |msql01|MySql |

    Scenario Outline: Entities and links
        Given a new graph
        And there are <nbe> entities in graph
        And there are <nbl> links in graph
        When we convert the graph using the <kernel> conversion core
        Then the resulting script looks like `assets/el/<name>.sql`

        Examples:
            |nbe|nbl|name  |kernel|
            |4  |1  |msql01|MySql |

    Scenario Outline: Full database
        Given a new graph
        And there are 5 entities in graph
        And the entity 1 is linked with 3 via l1
        And the entity 1 is linked with 4 via l1
        And the entity 2 is linked with 4 via l2
        And we want to name the database "<name>"
        When we convert the graph using the <kernel> conversion core
        Then the resulting script looks like `assets/db/<name>.sql`

        Examples:
            |name  |kernel|
            |msql01|MySql |
