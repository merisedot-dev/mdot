Feature: Writing down SQL scripts.

    Scenario Outline: Empty database
        Given a new graph
        And we want to name the database "<name>"
        When we convert the graph using the <kernel> conversion core
        Then the resulting script looks like `assets/empty/<name>.sql`

        Examples:
            |name|kernel|

    Scenario Outline: Only entities
        Given a new graph
        And there are <nbe> entities in graph
        When we convert the graph using the <kernel> conversion core

        Examples:
            |nbe|name|kernel|

    Scenario Outline: Entities and links
        Given a new graph
        And there are <nbe> entities in graph
        And there are <nbl> links in graph
        And we want to name the database "<name>"
        When we convert the graph using the <kernel> conversion core
        Then the resulting script looks like `assets/el/<name>.sql`

        Examples:
            |nbe|nbl|name|kernel|
