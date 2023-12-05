"use strict";

(self["webpackChunk"] = self["webpackChunk"] || []).push([ [ 717 ], {
    "./src/layouts/index.tsx": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            default: function() {
                return Layout;
            }
        });
        var umi__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./src/.umi-production/exports.ts");
        var _index_css_modules__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("./src/layouts/index.css?modules");
        var react_jsx_runtime__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__("./node_modules/.pnpm/react@18.2.0/node_modules/react/jsx-runtime.js");
        function Layout() {
            return (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_2__.jsxs)("div", {
                className: _index_css_modules__WEBPACK_IMPORTED_MODULE_1__["default"].navs,
                children: [ (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_2__.jsxs)("ul", {
                    children: [ (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_2__.jsx)("li", {
                        children: (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_2__.jsx)(umi__WEBPACK_IMPORTED_MODULE_0__.Link, {
                            to: "/",
                            children: "555555"
                        })
                    }), (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_2__.jsx)("li", {
                        children: (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_2__.jsx)(umi__WEBPACK_IMPORTED_MODULE_0__.Link, {
                            to: "/docs",
                            children: "Docs"
                        })
                    }), (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_2__.jsx)("li", {
                        children: (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_2__.jsx)("a", {
                            href: "https://github.com/umijs/umi",
                            children: "Github"
                        })
                    }) ]
                }), (0, react_jsx_runtime__WEBPACK_IMPORTED_MODULE_2__.jsx)(umi__WEBPACK_IMPORTED_MODULE_0__.Outlet, {}) ]
            });
        }
    },
    "./src/layouts/index.css?modules": function(__unused_webpack_module, __webpack_exports__) {
        __webpack_exports__["default"] = {};
    },
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
    }
} ]);
//# sourceMappingURL=layouts__index.f1ad91c2.async.js.map