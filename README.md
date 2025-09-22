# dx-collection
SSCCE managing collections using dioxus 0.7 stores

Goal is to have access to an abstraction such as `Collection<T, U>` where T is the item type and U is the identifier type (e.g. usize, Hash key)
Collection internally leverages dioxus Stores.
Collection should keep inner store fields private, in order to prevent users from messing up the selected item (e.g. out of bounds index) by providing safe methods to select and get the selected item
