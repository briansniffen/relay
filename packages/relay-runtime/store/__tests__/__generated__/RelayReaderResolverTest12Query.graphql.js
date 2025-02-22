/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @generated SignedSource<<006fe8f63715b11eba4f12b570cae418>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { ConcreteRequest, Query } from 'relay-runtime';
type UserAlwaysThrowsResolver$key = any;
import userAlwaysThrowsResolver from "../resolvers/UserAlwaysThrowsResolver.js";
// Type assertion validating that `userAlwaysThrowsResolver` resolver is correctly implemented.
// A type error here indicates that the type signature of the resolver module is incorrect.
(userAlwaysThrowsResolver: (
  rootKey: UserAlwaysThrowsResolver$key, 
) => mixed);
export type RelayReaderResolverTest12Query$variables = {||};
export type RelayReaderResolverTest12Query$data = {|
  +me: ?{|
    +always_throws: ?$Call<<R>((...empty[]) => R) => R, typeof userAlwaysThrowsResolver>,
  |},
|};
export type RelayReaderResolverTest12Query = {|
  response: RelayReaderResolverTest12Query$data,
  variables: RelayReaderResolverTest12Query$variables,
|};
*/

var node/*: ConcreteRequest*/ = {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "RelayReaderResolverTest12Query",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "User",
        "kind": "LinkedField",
        "name": "me",
        "plural": false,
        "selections": [
          {
            "alias": null,
            "args": null,
            "fragment": {
              "args": null,
              "kind": "FragmentSpread",
              "name": "UserAlwaysThrowsResolver"
            },
            "kind": "RelayResolver",
            "name": "always_throws",
            "resolverModule": require('./../resolvers/UserAlwaysThrowsResolver.js'),
            "path": "me.always_throws"
          }
        ],
        "storageKey": null
      }
    ],
    "type": "Query",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "RelayReaderResolverTest12Query",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "User",
        "kind": "LinkedField",
        "name": "me",
        "plural": false,
        "selections": [
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "__typename",
            "storageKey": null
          },
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "id",
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ]
  },
  "params": {
    "cacheID": "1a6c88e5b2da3c50f3c937e3501f55c8",
    "id": null,
    "metadata": {},
    "name": "RelayReaderResolverTest12Query",
    "operationKind": "query",
    "text": "query RelayReaderResolverTest12Query {\n  me {\n    ...UserAlwaysThrowsResolver\n    id\n  }\n}\n\nfragment UserAlwaysThrowsResolver on User {\n  __typename\n}\n"
  }
};

if (__DEV__) {
  (node/*: any*/).hash = "1a4d184b7854f9dced14abd8a720e85b";
}

module.exports = ((node/*: any*/)/*: Query<
  RelayReaderResolverTest12Query$variables,
  RelayReaderResolverTest12Query$data,
>*/);
