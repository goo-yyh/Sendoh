"use strict";

(self["webpackChunk"] = self["webpackChunk"] || []).push([ [ 866 ], {
    "./src/pages/A.tsx": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        var react_jsx_runtime__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./node_modules/.pnpm/react@18.2.0/node_modules/react/jsx-runtime.js");
        var C = function C() {
            return (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_0__.jsx)("div", {
                children: "C"
            });
        };
        __webpack_exports__["default"] = C;
    },
    "./src/pages/index.tsx": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            default: function() {
                return HomePage;
            }
        });
        var _assets_yay_jpg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./src/assets/yay.jpg");
        var _A__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("./src/pages/A.tsx");
        __webpack_require__("./src/pages/index.css");
        var _util__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__("./src/pages/util.ts");
        var react_jsx_runtime__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__("./node_modules/.pnpm/react@18.2.0/node_modules/react/jsx-runtime.js");
        function HomePage() {
            return (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_4__.jsxs)("div", {
                children: [ (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_4__.jsx)(_A__WEBPACK_IMPORTED_MODULE_1__["default"], {}), (0, 
                react_jsx_runtime__WEBPACK_IMPORTED_MODULE_4__.jsx)("h2", {
                    children: "Yay! Welcome to umi!111777222111888333444888667776888"
                }), (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_4__.jsx)("p", {
                    children: (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_4__.jsx)("img", {
                        src: _assets_yay_jpg__WEBPACK_IMPORTED_MODULE_0__,
                        width: "388"
                    })
                }), (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_4__.jsx)("p", {
                    children: (0, _util__WEBPACK_IMPORTED_MODULE_3__.foo)()
                }) ]
            });
        }
    },
    "./src/pages/util.ts": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.d(__webpack_exports__, {
            foo: function() {
                return foo;
            }
        });
        function foo() {
            return "bar";
        }
        console.log(1111);
    },
    "./src/pages/index.css": function() {},
    "./node_modules/.pnpm/react@18.2.0/node_modules/react/cjs/react-jsx-runtime.production.min.js": function(__unused_webpack_module, exports, __webpack_require__) {
        /**
 * @license React
 * react-jsx-runtime.production.min.js
 *
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
        var f = __webpack_require__("./node_modules/.pnpm/react@18.2.0/node_modules/react/index.js"), k = Symbol.for("react.element"), l = Symbol.for("react.fragment"), m = Object.prototype.hasOwnProperty, n = f.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED.ReactCurrentOwner, p = {
            key: !0,
            ref: !0,
            __self: !0,
            __source: !0
        };
        function q(c, a, g) {
            var b, d = {}, e = null, h = null;
            void 0 !== g && (e = "" + g);
            void 0 !== a.key && (e = "" + a.key);
            void 0 !== a.ref && (h = a.ref);
            for (b in a) m.call(a, b) && !p.hasOwnProperty(b) && (d[b] = a[b]);
            if (c && c.defaultProps) for (b in a = c.defaultProps, a) void 0 === d[b] && (d[b] = a[b]);
            return {
                $$typeof: k,
                type: c,
                key: e,
                ref: h,
                props: d,
                _owner: n.current
            };
        }
        l;
        exports.jsx = q;
        exports.jsxs = q;
    },
    "./node_modules/.pnpm/react@18.2.0/node_modules/react/jsx-runtime.js": function(module, __unused_webpack_exports, __webpack_require__) {
        if (true) module.exports = __webpack_require__("./node_modules/.pnpm/react@18.2.0/node_modules/react/cjs/react-jsx-runtime.production.min.js");
    },
    "./src/assets/yay.jpg": function(module, __unused_webpack_exports, __webpack_require__) {
        module.exports = __webpack_require__.p + "static/yay.7d162f31.jpg";
    }
} ]);
//# sourceMappingURL=p__index.8ab623f4.async.js.map