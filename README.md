# Bilbiographix

This project is part of the [snekdown](https://github.com/Trivernis/snekdown) parser project and encapsulates the bibliography handling.


## Bibliography types and fields

| Type          | Required Fields                               | Optional Fields                                    |
| ------------- | --------------------------------------------- | -------------------------------------------------- |
| article       | key, author, title, journal, date             | volume, number, pages, url                         |
| book          | key, author, title, publisher, date           | volume, series, address, edition, url              |
| booklet       | key, title                                    | author, how_published, address, date               |
| in_book       | key, author, title, position, publisher, date | volume, series, address, edition                   |
| in_collection | key, author, title, publisher, date           | editor, volume, series, position, address, edition |
| manual        | key, title                                    | author, organization, address, edition, date       |
| misc          | key                                           | author, title, date, url, how_published            |
| repository    | key, author, title                            | url, cms, license, accessed_at                     |
| tech_report   | key, author, title, institution, date         | number, address                                    |
| thesis        | key, author, title, school, date              | address                                            |
| unpublished   | key, author, title                            | date                                               |
| website       | key, url                                      | title, author, accessed_at, date                   |


## License

Distributed under Apache 2.0 License. See [LICENSE](https://github.com/Trivernis/bibliographix/blob/main/LICENSE) for more information.