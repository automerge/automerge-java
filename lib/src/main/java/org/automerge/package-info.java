/**
 * A library of data structures for building collaborative, local-first
 * applications
 *
 * <p>
 * The idea of automerge is to provide a data structure which is quite general,
 * - consisting of nested key/value maps and/or lists - which can be modified
 * entirely locally but which can at any time be merged with other instances of
 * the same data structure. This data structure is represented by the
 * {@link Document} class.
 *
 * <p>
 * In addition to the core data structure this library also provide an
 * implementation of a sync protocol which can be used over any reliable
 * in-order transport; and an efficient binary storage format.
 */
package org.automerge;
