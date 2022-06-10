(self.webpackChunk=self.webpackChunk||[]).push([[73404],{3905:(e,t,n)=>{"use strict";n.r(t),n.d(t,{MDXContext:()=>l,MDXProvider:()=>m,mdx:()=>h,useMDXComponents:()=>d,withMDXComponents:()=>u});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(){return i=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var r in n)Object.prototype.hasOwnProperty.call(n,r)&&(e[r]=n[r])}return e},i.apply(this,arguments)}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function c(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var l=r.createContext({}),u=function(e){return function(t){var n=d(t.components);return r.createElement(e,i({},t,{components:n}))}},d=function(e){var t=r.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):c(c({},t),e)),n},m=function(e){var t=d(e.components);return r.createElement(l.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},f=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,i=e.originalType,o=e.parentName,l=s(e,["components","mdxType","originalType","parentName"]),u=d(n),m=a,f=u["".concat(o,".").concat(m)]||u[m]||p[m]||i;return n?r.createElement(f,c(c({ref:t},l),{},{components:n})):r.createElement(f,c({ref:t},l))}));function h(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=n.length,o=new Array(i);o[0]=f;var c={};for(var s in t)hasOwnProperty.call(t,s)&&(c[s]=t[s]);c.originalType=e,c.mdxType="string"==typeof e?e:a,o[1]=c;for(var l=2;l<i;l++)o[l]=n[l];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}f.displayName="MDXCreateElement"},36742:(e,t,n)=>{"use strict";n.r(t),n.d(t,{default:()=>p});var r=n(79973),a=n(67294),i=n(73727),o=n(52263),c=n(13919),s=n(10412),l=(0,a.createContext)({collectLink:function(){}}),u=n(44996),d=n(18780),m=["isNavLink","to","href","activeClassName","isActive","data-noBrokenLinkCheck","autoAddBaseUrl"];const p=function(e){var t,n,p=e.isNavLink,f=e.to,h=e.href,v=e.activeClassName,g=e.isActive,y=e["data-noBrokenLinkCheck"],b=e.autoAddBaseUrl,w=void 0===b||b,x=(0,r.Z)(e,m),C=(0,o.default)().siteConfig,k=C.trailingSlash,O=C.baseUrl,N=(0,u.useBaseUrlUtils)().withBaseUrl,E=(0,a.useContext)(l),T=f||h,R=(0,c.Z)(T),j=null==T?void 0:T.replace("pathname://",""),L=void 0!==j?(n=j,w&&function(e){return e.startsWith("/")}(n)?N(n):n):void 0;L&&R&&(L=(0,d.applyTrailingSlash)(L,{trailingSlash:k,baseUrl:O}));var P=(0,a.useRef)(!1),I=p?i.OL:i.rU,A=s.default.canUseIntersectionObserver,U=(0,a.useRef)();(0,a.useEffect)((function(){return!A&&R&&null!=L&&window.docusaurus.prefetch(L),function(){A&&U.current&&U.current.disconnect()}}),[U,L,A,R]);var D=null!==(t=null==L?void 0:L.startsWith("#"))&&void 0!==t&&t,M=!L||!R||D;return L&&R&&!D&&!y&&E.collectLink(L),M?a.createElement("a",Object.assign({href:L},T&&!R&&{target:"_blank",rel:"noopener noreferrer"},x)):a.createElement(I,Object.assign({},x,{onMouseEnter:function(){P.current||null==L||(window.docusaurus.preload(L),P.current=!0)},innerRef:function(e){var t,n;A&&e&&R&&(t=e,n=function(){null!=L&&window.docusaurus.prefetch(L)},U.current=new window.IntersectionObserver((function(e){e.forEach((function(e){t===e.target&&(e.isIntersecting||e.intersectionRatio>0)&&(U.current.unobserve(t),U.current.disconnect(),n())}))})),U.current.observe(t))},to:L||""},p&&{isActive:g,activeClassName:v}))}},13919:(e,t,n)=>{"use strict";function r(e){return!0===/^(\w*:|\/\/)/.test(e)}function a(e){return void 0!==e&&!r(e)}n.d(t,{b:()=>r,Z:()=>a})},44996:(e,t,n)=>{"use strict";n.r(t),n.d(t,{useBaseUrlUtils:()=>i,default:()=>o});var r=n(52263),a=n(13919);function i(){var e=(0,r.default)().siteConfig,t=(e=void 0===e?{}:e).baseUrl,n=void 0===t?"/":t,i=e.url;return{withBaseUrl:function(e,t){return function(e,t,n,r){var i=void 0===r?{}:r,o=i.forcePrependBaseUrl,c=void 0!==o&&o,s=i.absolute,l=void 0!==s&&s;if(!n)return n;if(n.startsWith("#"))return n;if((0,a.b)(n))return n;if(c)return t+n;var u=n.startsWith(t)?n:t+n.replace(/^\//,"");return l?e+u:u}(i,n,e,t)}}}function o(e,t){return void 0===t&&(t={}),(0,i().withBaseUrl)(e,t)}},8802:(e,t)=>{"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.default=function(e,t){var n=t.trailingSlash,r=t.baseUrl;if(e.startsWith("#"))return e;if(void 0===n)return e;var a,i=e.split(/[#?]/)[0],o="/"===i||i===r?i:(a=i,n?function(e){return e.endsWith("/")?e:e+"/"}(a):function(e){return e.endsWith("/")?e.slice(0,-1):e}(a));return e.replace(i,o)}},18780:function(e,t,n){"use strict";var r=this&&this.__importDefault||function(e){return e&&e.__esModule?e:{default:e}};Object.defineProperty(t,"__esModule",{value:!0}),t.uniq=t.applyTrailingSlash=void 0;var a=n(8802);Object.defineProperty(t,"applyTrailingSlash",{enumerable:!0,get:function(){return r(a).default}});var i=n(29964);Object.defineProperty(t,"uniq",{enumerable:!0,get:function(){return r(i).default}})},29964:(e,t)=>{"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.default=function(e){return Array.from(new Set(e))}},68629:(e,t,n)=>{"use strict";n.d(t,{Z:()=>p});var r=n(36742),a=n(44256),i=n(67294);function o(){var e=window.encodeURI(JSON.stringify({title:"Feedback about "+window.location.pathname,description:"**!!! Required !!!**\n\nPlease modify the task description to let us know how the docs can be improved.\n\n**Please do not ask support questions via this form! Instead, ask in fburl.com/relay_support**",tag_ids:{add:[0xac96423e5b680,0x64079768ac750]}}));window.open("https://www.internalfb.com/tasks/?n="+e)}function c(e){var t=e.children;return i.createElement("div",{className:"docsRating",id:"docsRating"},i.createElement("hr",null),t)}var s=function(){var e=i.useState(!1),t=e[0],n=e[1],r=function(e){n(!0),function(e){window.ga&&window.ga("send",{hitType:"event",eventCategory:"button",eventAction:"feedback",eventValue:e})}(e)};return t?"Thank you for letting us know!":i.createElement(i.Fragment,null,"Is this page useful?",i.createElement("svg",{className:"i_thumbsup",alt:"Like",id:"docsRating-like",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 81.13 89.76",onClick:function(){return r(1)}},i.createElement("path",{d:"M22.9 6a18.57 18.57 0 002.67 8.4 25.72 25.72 0 008.65 7.66c3.86 2 8.67 7.13 13.51 11 3.86 3.11 8.57 7.11 11.54 8.45s13.59.26 14.64 1.17c1.88 1.63 1.55 9-.11 15.25-1.61 5.86-5.96 10.55-6.48 16.86-.4 4.83-2.7 4.88-10.93 4.88h-1.35c-3.82 0-8.24 2.93-12.92 3.62a68 68 0 01-9.73.5c-3.57 0-7.86-.08-13.25-.08-3.56 0-4.71-1.83-4.71-4.48h8.42a3.51 3.51 0 000-7H12.28a2.89 2.89 0 01-2.88-2.88 1.91 1.91 0 01.77-1.78h16.46a3.51 3.51 0 000-7H12.29c-3.21 0-4.84-1.83-4.84-4a6.41 6.41 0 011.17-3.78h19.06a3.5 3.5 0 100-7H9.75A3.51 3.51 0 016 42.27a3.45 3.45 0 013.75-3.48h13.11c5.61 0 7.71-3 5.71-5.52-4.43-4.74-10.84-12.62-11-18.71-.15-6.51 2.6-7.83 5.36-8.56m0-6a6.18 6.18 0 00-1.53.2c-6.69 1.77-10 6.65-9.82 14.5.08 5.09 2.99 11.18 8.52 18.09H9.74a9.52 9.52 0 00-6.23 16.9 12.52 12.52 0 00-2.07 6.84 9.64 9.64 0 003.65 7.7 7.85 7.85 0 00-1.7 5.13 8.9 8.9 0 005.3 8.13 6 6 0 00-.26 1.76c0 6.37 4.2 10.48 10.71 10.48h13.25a73.75 73.75 0 0010.6-.56 35.89 35.89 0 007.58-2.18 17.83 17.83 0 014.48-1.34h1.35c4.69 0 7.79 0 10.5-1 3.85-1.44 6-4.59 6.41-9.38.2-2.46 1.42-4.85 2.84-7.62a41.3 41.3 0 003.42-8.13 48 48 0 001.59-10.79c.1-5.13-1-8.48-3.35-10.55-2.16-1.87-4.64-1.87-9.6-1.88a46.86 46.86 0 01-6.64-.29c-1.92-.94-5.72-4-8.51-6.3l-1.58-1.28c-1.6-1.3-3.27-2.79-4.87-4.23-3.33-3-6.47-5.79-9.61-7.45a20.2 20.2 0 01-6.43-5.53 12.44 12.44 0 01-1.72-5.36 6 6 0 00-6-5.86z"})),i.createElement("svg",{className:"i_thumbsdown",alt:"Dislike",id:"docsRating-dislike",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 81.13 89.76",onClick:function(){return r(0)}},i.createElement("path",{d:"M22.9 6a18.57 18.57 0 002.67 8.4 25.72 25.72 0 008.65 7.66c3.86 2 8.67 7.13 13.51 11 3.86 3.11 8.57 7.11 11.54 8.45s13.59.26 14.64 1.17c1.88 1.63 1.55 9-.11 15.25-1.61 5.86-5.96 10.55-6.48 16.86-.4 4.83-2.7 4.88-10.93 4.88h-1.35c-3.82 0-8.24 2.93-12.92 3.62a68 68 0 01-9.73.5c-3.57 0-7.86-.08-13.25-.08-3.56 0-4.71-1.83-4.71-4.48h8.42a3.51 3.51 0 000-7H12.28a2.89 2.89 0 01-2.88-2.88 1.91 1.91 0 01.77-1.78h16.46a3.51 3.51 0 000-7H12.29c-3.21 0-4.84-1.83-4.84-4a6.41 6.41 0 011.17-3.78h19.06a3.5 3.5 0 100-7H9.75A3.51 3.51 0 016 42.27a3.45 3.45 0 013.75-3.48h13.11c5.61 0 7.71-3 5.71-5.52-4.43-4.74-10.84-12.62-11-18.71-.15-6.51 2.6-7.83 5.36-8.56m0-6a6.18 6.18 0 00-1.53.2c-6.69 1.77-10 6.65-9.82 14.5.08 5.09 2.99 11.18 8.52 18.09H9.74a9.52 9.52 0 00-6.23 16.9 12.52 12.52 0 00-2.07 6.84 9.64 9.64 0 003.65 7.7 7.85 7.85 0 00-1.7 5.13 8.9 8.9 0 005.3 8.13 6 6 0 00-.26 1.76c0 6.37 4.2 10.48 10.71 10.48h13.25a73.75 73.75 0 0010.6-.56 35.89 35.89 0 007.58-2.18 17.83 17.83 0 014.48-1.34h1.35c4.69 0 7.79 0 10.5-1 3.85-1.44 6-4.59 6.41-9.38.2-2.46 1.42-4.85 2.84-7.62a41.3 41.3 0 003.42-8.13 48 48 0 001.59-10.79c.1-5.13-1-8.48-3.35-10.55-2.16-1.87-4.64-1.87-9.6-1.88a46.86 46.86 0 01-6.64-.29c-1.92-.94-5.72-4-8.51-6.3l-1.58-1.28c-1.6-1.3-3.27-2.79-4.87-4.23-3.33-3-6.47-5.79-9.61-7.45a20.2 20.2 0 01-6.43-5.53 12.44 12.44 0 01-1.72-5.36 6 6 0 00-6-5.86z"})))},l=function(){return i.createElement("p",null,"Let us know how these docs can be improved by",i.createElement("a",{className:"button",role:"button",tabIndex:0,onClick:o},"Filing a task"))},u=function(){return i.createElement("p",null,"Help us make the site even better by"," ",i.createElement(r.default,{to:"https://www.surveymonkey.com/r/FYC9TCJ"},"answering a few quick questions"),".")},d=function(){return i.createElement(c,null,i.createElement(l,null),i.createElement(s,null),i.createElement(u,null))},m=function(){return i.createElement(c,null,i.createElement(s,null),i.createElement(u,null))};const p=function(){return(0,a.fbContent)({internal:i.createElement(d,null),external:i.createElement(m,null)})}},2600:(e,t,n)=>{"use strict";n.r(t),n.d(t,{frontMatter:()=>s,contentTitle:()=>l,metadata:()=>u,toc:()=>d,default:()=>p});var r=n(74034),a=n(79973),i=(n(67294),n(3905)),o=n(68629),c=["components"],s={id:"compiler-architecture",title:"Compiler Architecture",slug:"/principles-and-architecture/compiler-architecture/",description:"Relay compiler architecture guide",keywords:["compiler","architecture","transform"]},l=void 0,u={unversionedId:"principles-and-architecture/compiler-architecture",id:"version-v13.0.0/principles-and-architecture/compiler-architecture",isDocsHomePage:!1,title:"Compiler Architecture",description:"Relay compiler architecture guide",source:"@site/versioned_docs/version-v13.0.0/principles-and-architecture/compiler-architecture.md",sourceDirName:"principles-and-architecture",slug:"/principles-and-architecture/compiler-architecture/",permalink:"/docs/v13.0.0/principles-and-architecture/compiler-architecture/",editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v13.0.0/principles-and-architecture/compiler-architecture.md",tags:[],version:"v13.0.0",lastUpdatedBy:"Jordan Eldredge",lastUpdatedAt:1654886929,formattedLastUpdatedAt:"6/10/2022",frontMatter:{id:"compiler-architecture",title:"Compiler Architecture",slug:"/principles-and-architecture/compiler-architecture/",description:"Relay compiler architecture guide",keywords:["compiler","architecture","transform"]},sidebar:"version-v13.0.0/docs",previous:{title:"Architecture Overview",permalink:"/docs/v13.0.0/principles-and-architecture/architecture-overview/"},next:{title:"Runtime Architecture",permalink:"/docs/v13.0.0/principles-and-architecture/runtime-architecture/"}},d=[{value:"Data Flow",id:"data-flow",children:[],level:2},{value:"Data Types &amp; Modules",id:"data-types--modules",children:[],level:2},{value:"Transforms",id:"transforms",children:[],level:2}],m={toc:d};function p(e){var t=e.components,n=(0,a.Z)(e,c);return(0,i.mdx)("wrapper",(0,r.Z)({},m,n,{components:t,mdxType:"MDXLayout"}),(0,i.mdx)("p",null,"The compiler is a set of modules designed to extract GraphQL documents from across a codebase, transform/optimize them, and generate build artifacts. Examples of common types of artifacts include optimized GraphQL to persist to your server, runtime representations of the queries for use with GraphQL clients such as the Relay runtime, or generated source code for use with GraphQL frameworks for compiled languages (Java/Swift/etc)."),(0,i.mdx)("h2",{id:"data-flow"},"Data Flow"),(0,i.mdx)("p",null,"The high-level flow of data through the compiler is represented in the following diagram:"),(0,i.mdx)("pre",null,(0,i.mdx)("code",{parentName:"pre"},"\n                   \u250c\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2510\u250c\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2510\n                   \u2502   GraphQL   \u2502\u2502   Schema    \u2502\n                   \u2514\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2518\u2514\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2518\n                          \u2502              \u2502              parse\n                          \u2514\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u252c\u2500\u2500\u2500\u2500\u2500\u2500\u2518\n                                  \u25bc\n                   \u250c\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2510\n                   \u2502      CompilerContext       \u2502\n                   \u2502                            \u2502\n                   \u2502   \u250c\u2500\u2500\u2500\u2500\u2500\u2510 \u250c\u2500\u2500\u2500\u2500\u2500\u2510 \u250c\u2500\u2500\u2500\u2500\u2500\u2510  \u2502\u2500\u2500\u2510\n                   \u2502   \u2502 IR  \u2502 \u2502 IR  \u2502 \u2502 ... \u2502  \u2502  \u2502\n                   \u2502   \u2514\u2500\u2500\u2500\u2500\u2500\u2518 \u2514\u2500\u2500\u2500\u2500\u2500\u2518 \u2514\u2500\u2500\u2500\u2500\u2500\u2518  \u2502  \u2502\n                   \u2514\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2518  \u2502  transform/\n                          \u2502    \u2502      \u25b2            \u2502   optimize\n                          \u2502    \u2502      \u2514\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2518\n                          \u2502    \u2502\n                          \u2502    \u2514\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2510\n                          \u2502  print        \u2502  codegen\n                          \u25bc               \u25bc\n                   \u250c\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2510 \u250c\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2510\n                   \u2502   GraphQL   \u2502 \u2502  Artifacts  \u2502\n                   \u2514\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2518 \u2514\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2518\n")),(0,i.mdx)("ol",null,(0,i.mdx)("li",{parentName:"ol"},'GraphQL text is extracted from source files and "parsed" into an intermediate representation (IR) using information from the schema.'),(0,i.mdx)("li",{parentName:"ol"},"The set of IR documents forms a CompilerContext, which is then transformed and optimized."),(0,i.mdx)("li",{parentName:"ol"},"Finally, GraphQL is printed (e.g. to files, saved to a database, etc) and any artifacts are generated.")),(0,i.mdx)("h2",{id:"data-types--modules"},"Data Types & Modules"),(0,i.mdx)("p",null,"The compiler module is composed of a set of core building blocks as well as a helper that packages them together in an easy to use API. Some of the main data types and modules in the compiler are as follows:"),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"IR")," (Intermediate Representation): an (effectively immutable) representation of a GraphQL document (query, fragment, field, etc) as a tree structure, including type information from a schema. Compared to the standard GraphQL AST (produced by e.g. ",(0,i.mdx)("inlineCode",{parentName:"li"},"graphql-js"),") the main difference is that it encodes more of the semantics of GraphQL. For example, conditional branches (",(0,i.mdx)("inlineCode",{parentName:"li"},"@include")," and ",(0,i.mdx)("inlineCode",{parentName:"li"},"@skip"),") are represented directly, making it easier to target optimizations for these directives (One such optimization is to merge sibling fields with the same condition, potentially reducing the number of conditionals that must be evaluated at runtime)."),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"CompilerContext"),": an immutable representation of a corpus of GraphQL documents. It contains the schema and a mapping of document names to document representations (as IR, see above)."),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"Transform"),': a "map"-like function that accepts a ',(0,i.mdx)("inlineCode",{parentName:"li"},"CompilerContext")," as input and returns a new, modified context as output. Examples below."),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"Parser"),": Converts a GraphQL schema and raw GraphQL text into typed IR objects."),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"Printer"),": a function that accepts IR and converts it to a GraphQL string.")),(0,i.mdx)("p",null,"The ",(0,i.mdx)("inlineCode",{parentName:"p"},"RelayCompiler")," module is a helper class that demonstrates one way of combining these primitives. It takes IR transforms, and given IR definitions, constructs a CompilerContext from them, transforming them, and generating output artifacts intended for use with Relay runtime."),(0,i.mdx)("h2",{id:"transforms"},"Transforms"),(0,i.mdx)("p",null,"One of the main goals of the compiler is to provide a consistent platform for writing tools that transform or optimize GraphQL. This includes the ability to experiment with new directives by transforming them away at compile time. Transform functions should typically perform a single type of modification - it's expected that an app will have multiple transforms configured in the compiler instance."),(0,i.mdx)("p",null,"Here are a few examples of some of the included transforms:"),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"FlattenTransform"),": Reduces extraneous levels of indirection in a query, inlining fields from anonymous fragments wherever they match the parent type. This can be beneficial when generating code to read the results of a query or process query results, as it reduces duplicate field processing. For example:")),(0,i.mdx)("pre",null,(0,i.mdx)("code",{parentName:"pre"},"# before: `id` is processed twice\nfoo { # type FooType\n   id\n   ... on FooType { # matches the parent type, so this is extraneous\n     id\n   }\n }\n\n # after: `id` is processed once\n foo {\n   id\n }\n")),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"SkipRedundantNodeTransform"),": A more advanced version of flattening, this eliminates more complex cases of field duplication such as when a field is fetched both unconditionally and conditionally, or is fetched by two different sub-fragments. For example:")),(0,i.mdx)("pre",null,(0,i.mdx)("code",{parentName:"pre"},"# before: `id` processed up to 2x\nfoo {\n  bar {\n    id\n  }\n  ... on FooType @include(if: $cond) { # can't be flattened due to conditional\n    id # but this field is guaranteed to be fetched regardless\n  }\n}\n\n# after: `id` processed at most once\nfoo {\n  bar {\n    id\n  }\n}\n")),(0,i.mdx)(o.Z,{mdxType:"DocsRating"}))}p.isMDXComponent=!0}}]);