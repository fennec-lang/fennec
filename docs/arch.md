# Architecture musings

link to world: arc[world]

world:

- version
- map[(path, source), arc[module]]
  - or map[module-id, arc[module]] (slotmap)

module:

- (path, source)
  - stable numeric ID
- version
- map[(path, source), arc[package]]
  - or map[package-id, arc[package]] (slotmap)

package:

- (path, source)
  - stable numeric ID
- version
- map[item-id, arc[item]], slotmap

item:

- (item-id)
- version
- data?

- create new item                        `O(item)                 const`
- create new package linking to item     `O(items in package)     100`
  - **no need to update references to item**, because they are indirect (/module/package/item-id)
  - we intern the string IDs or we use numeric IDs?
  - (optional) 64-bit module-id, (optional) 64-bit package-id, 64-bit item-id
    - 192 bits is a lot, 24 bytes = 1 string in fact
- create new module linking to package   `O(packages in module)   10`
- create new world linking to module     `O(modules in world)     100`

- consider q-cell?

- item: resolve current version in a package
- package: resolve current version in a module
- module: resolve current version in a world

