use biome_js_factory::make::jsx_child_list;
use biome_js_syntax::{
    AnyJsConstructorParameter, AnyJsFormalParameter, AnyJsObjectMember, AnyJsParameter,
    AnyJsxChild, JsConstructorParameterList, JsFormalParameter, JsLanguage, JsObjectMemberList,
    JsParameterList, JsSyntaxKind, JsSyntaxNode, JsVariableDeclaration, JsVariableDeclarator,
    JsVariableDeclaratorList, JsVariableStatement, JsxChildList,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutation};

pub trait JsBatchMutation {
    /// Removes the declarator, and:
    /// 1 - removes the statement if the declaration only has one declarator;
    /// 2 - removes commas around the declarator to keep the list valid.
    fn remove_js_variable_declarator(&mut self, declarator: &JsVariableDeclarator) -> bool;

    /// Removes the parameter, and:
    /// 1 - removes commas around the parameter to keep the list valid.
    fn remove_js_formal_parameter(&mut self, parameter: &JsFormalParameter) -> bool;

    /// Removes the object member, and:
    /// 1 - removes commas around the member to keep the list valid.
    fn remove_js_object_member(&mut self, parameter: &AnyJsObjectMember) -> bool;

    /// Transfer leading trivia to the next sibling.
    /// If there is no next sibling, then transfer to the previous sibling.
    /// Otherwise do nothings.
    fn transfer_leading_trivia_to_sibling(&mut self, node: &JsSyntaxNode);

    /// It attempts to add a new element after the given element
    ///
    /// The function appends the elements at the end if `after_element` isn't found
    fn add_jsx_elements_after_element<I>(
        &mut self,
        after_element: &AnyJsxChild,
        new_element: I,
    ) -> bool
    where
        I: IntoIterator<Item = AnyJsxChild>;

    /// It attempts to add a new element before the given element.
    fn add_jsx_elements_before_element<I>(
        &mut self,
        after_element: &AnyJsxChild,
        new_elements: I,
    ) -> bool
    where
        I: IntoIterator<Item = AnyJsxChild>;
}

fn remove_js_formal_parameter_from_js_parameter_list(
    batch: &mut BatchMutation<JsLanguage>,
    parameter: &JsFormalParameter,
    list: JsSyntaxNode,
) -> Option<bool> {
    let list = JsParameterList::cast(list)?;
    let mut elements = list.elements();

    // Find the parameter we want to remove
    // remove its trailing comma, if there is one
    let mut previous_element = None;
    for element in elements.by_ref() {
        if let Ok(node) = element.node() {
            match node {
                AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                    node,
                )) if node == parameter => {
                    batch.remove_node(node.clone());
                    if let Ok(Some(comma)) = element.trailing_separator() {
                        batch.remove_token(comma.clone());
                    }
                    break;
                }
                _ => {}
            }
        }
        previous_element = Some(element);
    }

    // if it is the last parameter of the list
    // removes the comma before this element
    if elements.next().is_none() {
        if let Some(element) = previous_element {
            if let Ok(Some(comma)) = element.trailing_separator() {
                batch.remove_token(comma.clone());
            }
        }
    }

    Some(true)
}

fn remove_js_formal_parameter_from_js_constructor_parameter_list(
    batch: &mut BatchMutation<JsLanguage>,
    parameter: &JsFormalParameter,
    list: JsSyntaxNode,
) -> Option<bool> {
    let list = JsConstructorParameterList::cast(list)?;
    let mut elements = list.elements();

    // Find the parameter we want to remove
    // remove its trailing comma, if there is one
    let mut previous_element = None;
    for element in elements.by_ref() {
        if let Ok(node) = element.node() {
            match node {
                AnyJsConstructorParameter::AnyJsFormalParameter(
                    AnyJsFormalParameter::JsFormalParameter(node),
                ) if node == parameter => {
                    batch.remove_node(node.clone());
                    if let Ok(Some(comma)) = element.trailing_separator() {
                        batch.remove_token(comma.clone());
                    }
                    break;
                }
                _ => {}
            }
        }
        previous_element = Some(element);
    }

    // if it is the last parameter of the list
    // removes the comma before this element
    if elements.next().is_none() {
        if let Some(element) = previous_element {
            if let Ok(Some(comma)) = element.trailing_separator() {
                batch.remove_token(comma.clone());
            }
        }
    }

    Some(true)
}

impl JsBatchMutation for BatchMutation<JsLanguage> {
    fn remove_js_variable_declarator(&mut self, declarator: &JsVariableDeclarator) -> bool {
        declarator
            .parent::<JsVariableDeclaratorList>()
            .and_then(|list| {
                let declaration = list.parent::<JsVariableDeclaration>()?;

                if list.syntax_list().len() == 1 {
                    let statement = declaration.parent::<JsVariableStatement>()?;
                    self.remove_node(statement);
                } else {
                    let mut elements = list.elements();

                    // Find the declarator we want to remove
                    // remove its trailing comma, if there is one
                    let mut previous_element = None;
                    for element in elements.by_ref() {
                        if let Ok(node) = element.node() {
                            if node == declarator {
                                self.remove_node(node.clone());
                                if let Ok(Some(comma)) = element.trailing_separator() {
                                    self.remove_token(comma.clone());
                                }
                                break;
                            }
                        }
                        previous_element = Some(element);
                    }

                    // if it is the last declarator of the list
                    // removes the comma before this element
                    let remove_previous_element_comma = match elements.next() {
                        Some(e) if e.node().is_err() => true,
                        None => true,
                        _ => false,
                    };

                    if remove_previous_element_comma {
                        if let Some(element) = previous_element {
                            if let Ok(Some(comma)) = element.trailing_separator() {
                                self.remove_token(comma.clone());
                            }
                        }
                    }
                }

                Some(true)
            })
            .unwrap_or(false)
    }

    fn remove_js_formal_parameter(&mut self, parameter: &JsFormalParameter) -> bool {
        parameter
            .syntax()
            .parent()
            .and_then(|parent| match parent.kind() {
                JsSyntaxKind::JS_PARAMETER_LIST => {
                    remove_js_formal_parameter_from_js_parameter_list(self, parameter, parent)
                }
                JsSyntaxKind::JS_CONSTRUCTOR_PARAMETER_LIST => {
                    remove_js_formal_parameter_from_js_constructor_parameter_list(
                        self, parameter, parent,
                    )
                }
                _ => None,
            })
            .unwrap_or(false)
    }

    fn remove_js_object_member(&mut self, member: &AnyJsObjectMember) -> bool {
        member
            .syntax()
            .parent()
            .and_then(|parent| {
                let parent = JsObjectMemberList::cast(parent)?;
                for element in parent.elements() {
                    if element.node() == Ok(member) {
                        self.remove_node(member.clone());
                        if let Ok(Some(comma)) = element.trailing_separator() {
                            self.remove_token(comma.clone());
                        }
                    }
                }

                Some(true)
            })
            .unwrap_or(false)
    }

    fn add_jsx_elements_after_element<I>(
        &mut self,
        after_element: &AnyJsxChild,
        new_elements: I,
    ) -> bool
    where
        I: IntoIterator<Item = AnyJsxChild>,
    {
        let old_list = after_element.parent::<JsxChildList>();
        if let Some(old_list) = &old_list {
            let jsx_child_list = {
                let mut new_items = vec![];
                let mut old_elements = old_list.into_iter();

                for old_element in old_elements.by_ref() {
                    let is_needle = old_element == *after_element;

                    new_items.push(old_element.clone());
                    if is_needle {
                        break;
                    }
                }

                new_items.extend(new_elements);
                new_items.extend(old_elements);

                jsx_child_list(new_items)
            };

            self.replace_node_discard_trivia(old_list.clone(), jsx_child_list);
            true
        } else {
            false
        }
    }

    fn add_jsx_elements_before_element<I>(
        &mut self,
        before_element: &AnyJsxChild,
        new_elements: I,
    ) -> bool
    where
        I: IntoIterator<Item = AnyJsxChild>,
    {
        let old_list = before_element
            .syntax()
            .parent()
            .and_then(JsxChildList::cast);
        if let Some(old_list) = &old_list {
            let jsx_child_list = {
                let mut new_items = vec![];
                let mut old_elements = old_list.into_iter().peekable();

                while let Some(next_element) = old_elements.peek() {
                    if next_element == before_element {
                        break;
                    }
                    new_items.push(next_element.clone());
                    old_elements.next();
                }

                new_items.extend(new_elements);
                new_items.extend(old_elements);

                jsx_child_list(new_items)
            };

            self.replace_node_discard_trivia(old_list.clone(), jsx_child_list);
            true
        } else {
            false
        }
    }

    fn transfer_leading_trivia_to_sibling(&mut self, node: &JsSyntaxNode) {
        let Some(pieces) = node.first_leading_trivia().map(|trivia| trivia.pieces()) else {
            return;
        };
        let (sibling, new_sibling) =
            if let Some(next_sibling) = node.last_token().and_then(|x| x.next_token()) {
                (
                    next_sibling.clone(),
                    next_sibling.prepend_trivia_pieces(pieces),
                )
            } else if let Some(prev_sibling) = node.first_token().and_then(|x| x.prev_token()) {
                (
                    prev_sibling.clone(),
                    prev_sibling.append_trivia_pieces(pieces),
                )
            } else {
                return;
            };
        self.replace_token_discard_trivia(sibling, new_sibling);
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_remove_ok;
    use biome_js_syntax::{AnyJsObjectMember, JsFormalParameter, JsVariableDeclarator};

    // Remove JsVariableDeclarator
    assert_remove_ok! {
        JsVariableDeclarator,
        ok_remove_variable_declarator_single,
            "let a;",
            "",
        ok_remove_variable_declarator_fist,
            "let a, b;",
            "let b;",
        ok_remove_variable_declarator_second,
            "let b, a;",
            "let b;",
        ok_remove_variable_declarator_second_trailling_comma,
            "let b, a,;",
            "let b;",
        ok_remove_variable_declarator_middle,
            "let b, a, c;",
            "let b, c;",
    }

    // Remove JsFormalParameter from functions
    assert_remove_ok! {
        JsFormalParameter,
        ok_remove_formal_parameter_single,
            "function f(a) {}",
            "function f() {}",
        ok_remove_formal_parameter_first,
            "function f(a, b) {}",
            "function f(b) {}",
        ok_remove_formal_parameter_second,
            "function f(b, a) {}",
            "function f(b) {}",
        ok_remove_formal_parameter_second_trailing_comma,
            "function f(b, a,) {}",
            "function f(b) {}",
        ok_remove_formal_parameter_middle,
            "function f(b, a, c) {}",
            "function f(b, c) {}",
    }

    // Remove JsFormalParameter from class constructors
    assert_remove_ok! {
        JsFormalParameter,
        ok_remove_formal_parameter_from_class_constructor_single,
            "class A { constructor(a) {} }",
            "class A { constructor() {} }",
        ok_remove_formal_parameter_from_class_constructor_first,
            "class A { constructor(a, b) {} }",
            "class A { constructor(b) {} }",
        ok_remove_formal_parameter_from_class_constructor_second,
            "class A { constructor(b, a) {} }",
            "class A { constructor(b) {} }",
        ok_remove_formal_parameter_from_class_constructor_second_trailing_comma,
            "class A { constructor(b, a,) {} }",
            "class A { constructor(b) {} }",
        ok_remove_formal_parameter_from_class_constructor_middle,
            "class A { constructor(b, a, c) {} }",
            "class A { constructor(b, c) {} }",
    }

    // Remove JsAnyObjectMember from object expression
    assert_remove_ok! {
        AnyJsObjectMember,
        ok_remove_first_member,
            "({ a: 1, b: 2 })",
            "({ b: 2 })",
        ok_remove_middle_member,
            "({ z: 1, a: 2, b: 3 })",
            "({ z: 1, b: 3 })",
        ok_remove_last_member,
            "({ z: 1, a: 2 })",
            "({ z: 1, })",
        ok_remove_last_member_trailing_comma,
            "({ z: 1, a: 2, })",
            "({ z: 1, })",
        ok_remove_only_member,
            "({ a: 2 })",
            "({ })",
    }
}
