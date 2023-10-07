// This file contains the list of all diagnostic categories for the Biome
// toolchain
//
// The `define_categories` macro is preprocessed in the build script for the
// crate in order to generate the static registry. The body of the macro
// consists of a list of key-value pairs defining the categories that have an
// associated hyperlink, then a list of string literals defining the remaining
// categories without a link.

// PLEASE, DON'T EDIT THIS FILE BY HAND.
// Use `just new-lintrule` to create a new rule.
// lint rules are lexicographically sorted and
// must be between `define_categories! {\n` and `\n    ;\n`.

define_categories! {
    "lint/a11y/noAccessKey": "https://biomejs.dev/linter/rules/no-access-key",
    "lint/a11y/noAriaUnsupportedElements": "https://biomejs.dev/linter/rules/no-aria-unsupported-elements",
    "lint/a11y/noAutofocus": "https://biomejs.dev/linter/rules/no-autofocus",
    "lint/a11y/noBlankTarget": "https://biomejs.dev/linter/rules/no-blank-target",
    "lint/a11y/noDistractingElements": "https://biomejs.dev/linter/rules/no-distracting-elements",
    "lint/a11y/noHeaderScope": "https://biomejs.dev/linter/rules/no-header-scope",
    "lint/a11y/noNoninteractiveElementToInteractiveRole": "https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role",
    "lint/a11y/noNoninteractiveTabindex": "https://biomejs.dev/linter/rules/no-noninteractive-tabindex",
    "lint/a11y/noPositiveTabindex": "https://biomejs.dev/linter/rules/no-positive-tabindex",
    "lint/a11y/noRedundantAlt": "https://biomejs.dev/linter/rules/no-redundant-alt",
    "lint/a11y/noRedundantRoles": "https://biomejs.dev/linter/rules/no-redundant-roles",
    "lint/a11y/noSvgWithoutTitle": "https://biomejs.dev/linter/rules/no-svg-without-title",
    "lint/a11y/useAltText": "https://biomejs.dev/linter/rules/use-alt-text",
    "lint/a11y/useAnchorContent": "https://biomejs.dev/linter/rules/use-anchor-content",
    "lint/a11y/useAriaPropsForRole": "https://biomejs.dev/linter/rules/use-aria-props-for-role",
    "lint/a11y/useButtonType": "https://biomejs.dev/linter/rules/use-button-type",
    "lint/a11y/useHeadingContent": "https://biomejs.dev/linter/rules/use-heading-content",
    "lint/a11y/useHtmlLang": "https://biomejs.dev/linter/rules/use-html-lang",
    "lint/a11y/useIframeTitle": "https://biomejs.dev/linter/rules/use-iframe-title",
    "lint/a11y/useKeyWithClickEvents": "https://biomejs.dev/linter/rules/use-key-with-click-events",
    "lint/a11y/useKeyWithMouseEvents": "https://biomejs.dev/linter/rules/use-key-with-mouse-events",
    "lint/a11y/useMediaCaption": "https://biomejs.dev/linter/rules/use-media-caption",
    "lint/a11y/useValidAnchor": "https://biomejs.dev/linter/rules/use-valid-anchor",
    "lint/a11y/useValidAriaProps": "https://biomejs.dev/linter/rules/use-valid-aria-props",
    "lint/a11y/useValidAriaValues": "https://biomejs.dev/linter/rules/use-valid-aria-values",
    "lint/a11y/useValidLang": "https://biomejs.dev/linter/rules/use-valid-lang",
    "lint/complexity/noBannedTypes": "https://biomejs.dev/linter/rules/no-banned-types",
    "lint/complexity/noExtraBooleanCast": "https://biomejs.dev/linter/rules/no-extra-boolean-cast",
    "lint/complexity/noForEach": "https://biomejs.dev/linter/rules/no-for-each",
    "lint/complexity/noMultipleSpacesInRegularExpressionLiterals": "https://biomejs.dev/linter/rules/no-multiple-spaces-in-regular-expression-literals",
    "lint/complexity/noStaticOnlyClass": "https://biomejs.dev/linter/rules/no-static-only-class",
    "lint/complexity/noUselessCatch": "https://biomejs.dev/linter/rules/no-useless-catch",
    "lint/complexity/noUselessConstructor": "https://biomejs.dev/linter/rules/no-useless-constructor",
    "lint/complexity/noUselessEmptyExport": "https://biomejs.dev/linter/rules/no-useless-empty-export",
    "lint/complexity/noUselessFragments": "https://biomejs.dev/linter/rules/no-useless-fragments",
    "lint/complexity/noUselessLabel": "https://biomejs.dev/linter/rules/no-useless-label",
    "lint/complexity/noUselessRename": "https://biomejs.dev/linter/rules/no-useless-rename",
    "lint/complexity/noUselessSwitchCase": "https://biomejs.dev/linter/rules/no-useless-switch-case",
    "lint/complexity/noUselessThisAlias": "https://biomejs.dev/linter/rules/no-useless-this-alias",
    "lint/complexity/noUselessTypeConstraint": "https://biomejs.dev/linter/rules/no-useless-type-constraint",
    "lint/complexity/noWith": "https://biomejs.dev/linter/rules/no-with",
    "lint/complexity/useFlatMap": "https://biomejs.dev/linter/rules/use-flat-map",
    "lint/complexity/useLiteralKeys": "https://biomejs.dev/linter/rules/use-literal-keys",
    "lint/complexity/useOptionalChain": "https://biomejs.dev/linter/rules/use-optional-chain",
    "lint/complexity/useSimpleNumberKeys": "https://biomejs.dev/linter/rules/use-simple-number-keys",
    "lint/complexity/useSimplifiedLogicExpression": "https://biomejs.dev/linter/rules/use-simplified-logic-expression",
    "lint/correctness/noChildrenProp": "https://biomejs.dev/linter/rules/no-children-prop",
    "lint/correctness/noConstAssign": "https://biomejs.dev/linter/rules/no-const-assign",
    "lint/correctness/noConstantCondition": "https://biomejs.dev/linter/rules/no-constant-condition",
    "lint/correctness/noConstructorReturn": "https://biomejs.dev/linter/rules/no-constructor-return",
    "lint/correctness/noEmptyPattern": "https://biomejs.dev/linter/rules/no-empty-pattern",
    "lint/correctness/noGlobalObjectCalls": "https://biomejs.dev/linter/rules/no-global-object-calls",
    "lint/correctness/noInnerDeclarations": "https://biomejs.dev/linter/rules/no-inner-declarations",
    "lint/correctness/noInvalidConstructorSuper": "https://biomejs.dev/linter/rules/no-invalid-constructor-super",
    "lint/correctness/noNewSymbol": "https://biomejs.dev/linter/rules/no-new-symbol",
    "lint/correctness/noNonoctalDecimalEscape": "https://biomejs.dev/linter/rules/no-nonoctal-decimal-escape",
    "lint/correctness/noPrecisionLoss": "https://biomejs.dev/linter/rules/no-precision-loss",
    "lint/correctness/noRenderReturnValue": "https://biomejs.dev/linter/rules/no-render-return-value",
    "lint/correctness/noSelfAssign": "https://biomejs.dev/linter/rules/no-self-assign",
    "lint/correctness/noSetterReturn": "https://biomejs.dev/linter/rules/no-setter-return",
    "lint/correctness/noStringCaseMismatch": "https://biomejs.dev/linter/rules/no-string-case-mismatch",
    "lint/correctness/noSwitchDeclarations": "https://biomejs.dev/linter/rules/no-switch-declarations",
    "lint/correctness/noUndeclaredVariables": "https://biomejs.dev/linter/rules/no-undeclared-variables",
    "lint/correctness/noUnnecessaryContinue": "https://biomejs.dev/linter/rules/no-unnecessary-continue",
    "lint/correctness/noUnreachable": "https://biomejs.dev/linter/rules/no-unreachable",
    "lint/correctness/noUnreachableSuper": "https://biomejs.dev/docs/linter/rules/no-unreachable-super",
    "lint/correctness/noUnsafeFinally": "https://biomejs.dev/linter/rules/no-unsafe-finally",
    "lint/correctness/noUnsafeOptionalChaining": "https://biomejs.dev/linter/rules/no-unsafe-optional-chaining",
    "lint/correctness/noUnusedLabels": "https://biomejs.dev/linter/rules/no-unused-labels",
    "lint/correctness/noUnusedVariables": "https://biomejs.dev/linter/rules/no-unused-variables",
    "lint/correctness/noVoidElementsWithChildren": "https://biomejs.dev/linter/rules/no-void-elements-with-children",
    "lint/correctness/noVoidTypeReturn": "https://biomejs.dev/linter/rules/no-void-type-return",
    "lint/correctness/useIsNan": "https://biomejs.dev/linter/rules/use-is-nan",
    "lint/correctness/useValidForDirection": "https://biomejs.dev/linter/rules/use-valid-for-direction",
    "lint/correctness/useYield": "https://biomejs.dev/linter/rules/use-yield",
    "lint/nursery/noAccumulatingSpread": "https://biomejs.dev/linter/rules/no-accumulating-spread",
    "lint/nursery/noApproximativeNumericConstant": "https://biomejs.dev/lint/rules/no-approximative-numeric-constant",
    "lint/nursery/noConfusingVoidType": "https://biomejs.dev/linter/rules/no-confusing-void-type",
    "lint/nursery/noDuplicateJsonKeys": "https://biomejs.dev/linter/rules/no-duplicate-json-keys",
    "lint/nursery/noEmptyBlock": "https://biomejs.dev/lint/rules/no-empty-block",
    "lint/nursery/noEmptyCharacterClassInRegex": "https://biomejs.dev/lint/rules/no-empty-character-class-in-regex",
    "lint/nursery/noExcessiveComplexity": "https://biomejs.dev/linter/rules/no-excessive-complexity",
    "lint/nursery/noFallthroughSwitchClause": "https://biomejs.dev/linter/rules/no-fallthrough-switch-clause",
    "lint/nursery/noGlobalIsFinite": "https://biomejs.dev/linter/rules/no-global-is-finite",
    "lint/nursery/noGlobalIsNan": "https://biomejs.dev/linter/rules/no-global-is-nan",
    "lint/nursery/noInvalidNewBuiltin": "https://biomejs.dev/lint/rules/no-invalid-new-builtin",
    "lint/nursery/noMisleadingInstantiator": "https://biomejs.dev/linter/rules/no-misleading-instantiator",
    "lint/nursery/noMisrefactoredShorthandAssign": "https://biomejs.dev/lint/rules/no-misrefactored-shorthand-assign",
    "lint/nursery/noUnusedImports": "https://biomejs.dev/lint/rules/no-unused-imports",
    "lint/nursery/noUselessElse": "https://biomejs.dev/lint/rules/no-useless-else",
    "lint/nursery/noVoid": "https://biomejs.dev/linter/rules/no-void",
    "lint/nursery/useArrowFunction": "https://biomejs.dev/linter/rules/use-arrow-function",
    "lint/nursery/useAsConstAssertion": "https://biomejs.dev/lint/rules/use-as-const-assertion",
    "lint/nursery/useBiomeSuppressionComment": "https://biomejs.dev/lint/rules/use-biome-suppression-comment",
    "lint/nursery/useCollapsedElseIf": "https://biomejs.dev/lint/rules/use-collapsed-else-if",
    "lint/nursery/useExhaustiveDependencies": "https://biomejs.dev/linter/rules/use-exhaustive-dependencies",
    "lint/nursery/useGroupedTypeImport": "https://biomejs.dev/linter/rules/use-grouped-type-import",
    "lint/nursery/useHookAtTopLevel": "https://biomejs.dev/linter/rules/use-hook-at-top-level",
    "lint/nursery/useImportRestrictions": "https://biomejs.dev/linter/rules/use-import-restrictions",
    "lint/nursery/useIsArray": "https://biomejs.dev/linter/rules/use-is-array",
    "lint/nursery/useShorthandAssign": "https://biomejs.dev/lint/rules/use-shorthand-assign",
    "lint/performance/noDelete": "https://biomejs.dev/linter/rules/no-delete",
    "lint/security/noDangerouslySetInnerHtml": "https://biomejs.dev/linter/rules/no-dangerously-set-inner-html",
    "lint/security/noDangerouslySetInnerHtmlWithChildren": "https://biomejs.dev/linter/rules/no-dangerously-set-inner-html-with-children",
    "lint/style/noArguments": "https://biomejs.dev/linter/rules/no-arguments",
    "lint/style/noCommaOperator": "https://biomejs.dev/linter/rules/no-comma-operator",
    "lint/style/noImplicitBoolean": "https://biomejs.dev/linter/rules/no-implicit-boolean",
    "lint/style/noInferrableTypes": "https://biomejs.dev/linter/rules/no-inferrable-types",
    "lint/style/noNamespace": "https://biomejs.dev/linter/rules/no-namespace",
    "lint/style/noNegationElse": "https://biomejs.dev/linter/rules/no-negation-else",
    "lint/style/noNonNullAssertion": "https://biomejs.dev/linter/rules/no-non-null-assertion",
    "lint/style/noParameterAssign": "https://biomejs.dev/linter/rules/no-parameter-assign",
    "lint/style/noParameterProperties": "https://biomejs.dev/linter/rules/no-parameter-properties",
    "lint/style/noRestrictedGlobals": "https://biomejs.dev/linter/rules/no-restricted-globals",
    "lint/style/noShoutyConstants": "https://biomejs.dev/linter/rules/no-shouty-constants",
    "lint/style/noUnusedTemplateLiteral": "https://biomejs.dev/linter/rules/no-unused-template-literal",
    "lint/style/noVar": "https://biomejs.dev/linter/rules/no-var",
    "lint/style/useBlockStatements": "https://biomejs.dev/linter/rules/use-block-statements",
    "lint/style/useConst": "https://biomejs.dev/linter/rules/use-const",
    "lint/style/useDefaultParameterLast": "https://biomejs.dev/linter/rules/use-default-parameter-last",
    "lint/style/useEnumInitializers": "https://biomejs.dev/linter/rules/use-enum-initializers",
    "lint/style/useExponentiationOperator": "https://biomejs.dev/linter/rules/use-exponentiation-operator",
    "lint/style/useFragmentSyntax": "https://biomejs.dev/linter/rules/use-fragment-syntax",
    "lint/style/useLiteralEnumMembers": "https://biomejs.dev/linter/rules/use-literal-enum-members",
    "lint/style/useNamingConvention": "https://biomejs.dev/linter/rules/use-naming-convention",
    "lint/style/useNumericLiterals": "https://biomejs.dev/linter/rules/use-numeric-literals",
    "lint/style/useSelfClosingElements": "https://biomejs.dev/linter/rules/use-self-closing-elements",
    "lint/style/useShorthandArrayType": "https://biomejs.dev/linter/rules/use-shorthand-array-type",
    "lint/style/useSingleCaseStatement": "https://biomejs.dev/linter/rules/use-single-case-statement",
    "lint/style/useSingleVarDeclarator": "https://biomejs.dev/linter/rules/use-single-var-declarator",
    "lint/style/useTemplate": "https://biomejs.dev/linter/rules/use-template",
    "lint/style/useWhile": "https://biomejs.dev/linter/rules/use-while",
    "lint/suspicious/noArrayIndexKey": "https://biomejs.dev/linter/rules/no-array-index-key",
    "lint/suspicious/noAssignInExpressions": "https://biomejs.dev/linter/rules/no-assign-in-expressions",
    "lint/suspicious/noAsyncPromiseExecutor": "https://biomejs.dev/linter/rules/no-async-promise-executor",
    "lint/suspicious/noCatchAssign": "https://biomejs.dev/linter/rules/no-catch-assign",
    "lint/suspicious/noClassAssign": "https://biomejs.dev/linter/rules/no-class-assign",
    "lint/suspicious/noCommentText": "https://biomejs.dev/linter/rules/no-comment-text",
    "lint/suspicious/noCompareNegZero": "https://biomejs.dev/linter/rules/no-compare-neg-zero",
    "lint/suspicious/noConfusingLabels": "https://biomejs.dev/linter/rules/no-confusing-labels",
    "lint/suspicious/noConsoleLog": "https://biomejs.dev/linter/rules/no-console-log",
    "lint/suspicious/noConstEnum": "https://biomejs.dev/linter/rules/no-const-enum",
    "lint/suspicious/noControlCharactersInRegex": "https://biomejs.dev/linter/rules/no-control-characters-in-regex",
    "lint/suspicious/noDebugger": "https://biomejs.dev/linter/rules/no-debugger",
    "lint/suspicious/noDoubleEquals": "https://biomejs.dev/linter/rules/no-double-equals",
    "lint/suspicious/noDuplicateCase": "https://biomejs.dev/linter/rules/no-duplicate-case",
    "lint/suspicious/noDuplicateClassMembers": "https://biomejs.dev/linter/rules/no-duplicate-class-members",
    "lint/suspicious/noDuplicateJsxProps": "https://biomejs.dev/linter/rules/no-duplicate-jsx-props",
    "lint/suspicious/noDuplicateObjectKeys": "https://biomejs.dev/linter/rules/no-duplicate-object-keys",
    "lint/suspicious/noDuplicateParameters": "https://biomejs.dev/linter/rules/no-duplicate-parameters",
    "lint/suspicious/noEmptyInterface": "https://biomejs.dev/linter/rules/no-empty-interface",
    "lint/suspicious/noExplicitAny": "https://biomejs.dev/linter/rules/no-explicit-any",
    "lint/suspicious/noExtraNonNullAssertion": "https://biomejs.dev/linter/rules/no-extra-non-null-assertion",
    "lint/suspicious/noFunctionAssign": "https://biomejs.dev/linter/rules/no-function-assign",
    "lint/suspicious/noImportAssign": "https://biomejs.dev/linter/rules/no-import-assign",
    "lint/suspicious/noLabelVar": "https://biomejs.dev/linter/rules/no-label-var",
    "lint/suspicious/noPrototypeBuiltins": "https://biomejs.dev/linter/rules/no-prototype-builtins",
    "lint/suspicious/noRedeclare": "https://biomejs.dev/linter/rules/no-redeclare",
    "lint/suspicious/noRedundantUseStrict": "https://biomejs.dev/linter/rules/no-redundant-use-strict",
    "lint/suspicious/noSelfCompare": "https://biomejs.dev/linter/rules/no-self-compare",
    "lint/suspicious/noShadowRestrictedNames": "https://biomejs.dev/linter/rules/no-shadow-restricted-names",
    "lint/suspicious/noSparseArray": "https://biomejs.dev/linter/rules/no-sparse-array",
    "lint/suspicious/noUnsafeDeclarationMerging": "https://biomejs.dev/linter/rules/no-unsafe-declaration-merging",
    "lint/suspicious/noUnsafeNegation": "https://biomejs.dev/linter/rules/no-unsafe-negation",
    "lint/suspicious/useDefaultSwitchClauseLast": "https://biomejs.dev/linter/rules/use-default-switch-clause-last",
    "lint/suspicious/useGetterReturn": "https://biomejs.dev/linter/rules/use-getter-return",
    "lint/suspicious/useNamespaceKeyword": "https://biomejs.dev/linter/rules/use-namespace-keyword",
    "lint/suspicious/useValidTypeof": "https://biomejs.dev/linter/rules/use-valid-typeof",
    ;
    // General categories
    "files/missingHandler",
    "format",
    "check",
    "ci",
    "configuration",
    "organizeImports",
    "migrate",
    "deserialize",
    "internalError/io",
    "internalError/fs",
    "internalError/panic",
    // parse categories
    "parse",
    "parse/noSuperWithoutExtends",
    "parse/noDuplicatePrivateClassMembers",

    // Lint groups
    "lint",
    "lint/a11y",
    "lint/complexity",
    "lint/correctness",
    "lint/nursery",
    "lint/performance",
    "lint/security",
    "lint/style",
    "lint/suspicious",

    // Suppression comments
    "suppressions/parse",
    "suppressions/unknownGroup",
    "suppressions/unknownRule",
    "suppressions/unused",
    "suppressions/deprecatedSuppressionComment",

    // Used in tests and examples
    "args/fileNotFound",
    "flags/invalid",
    "semanticTests",
}
