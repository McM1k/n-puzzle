/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   Node.hpp                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gboudrie <gboudrie@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2018/11/26 18:21:26 by gboudrie          #+#    #+#             */
/*   Updated: 2018/11/26 18:21:26 by gboudrie         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef NODE_HPP
# define NODE_HPP

# include <iostream>

class Node {
public:
    Node(std::list<auto> content, auto weight);
    Node(Node const &src) = default;
    virtual ~Node(void) = default;
    Node &operator=(Node const &rhs) = default;

    static void setSize(auto size);

    Node

private:
    Node(void) = default;

    static auto             _size;
    std::list<auto> const   _content;
    auto            const   _weight;
};

std::ostream &operator<<(std::ostream &o, Node const &i);

#endif