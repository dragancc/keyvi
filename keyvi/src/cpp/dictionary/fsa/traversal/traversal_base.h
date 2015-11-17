/* * keyvi - A key value store.
 *
 * Copyright 2015 Hendrik Muhs<hendrik.muhs@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */


/*
 * traversal_base.h
 *
 *  Created on: Nov 17, 2015
 *      Author: hendrik
 */

#ifndef TRAVERSAL_BASE_H_
#define TRAVERSAL_BASE_H_

//#define ENABLE_TRACING
#include "dictionary/util/trace.h"

namespace keyvi {
namespace dictionary {
namespace fsa {
namespace traversal {

struct Transition {
  Transition(uint64_t s, unsigned char l): state(s), label(l) {}

  uint64_t state;
  unsigned char label;
};

template<class TransitionT = Transition>
struct TraversalStatePayload {
  std::vector<TransitionT> transitions_;
  size_t position;
};

/**
 * A helper data structure to hold a state in graph traversal
 */
template<class TransitionT = Transition>
struct TraversalState {

  void Add(uint64_t s, unsigned char l) {
    payload_.transitions_.push_back(TransitionT(s, l));
  }

  void Add(uint64_t s, uint32_t w, unsigned char l) {
    payload_.transitions_.push_back(TransitionT(s, w, l));
  }

  uint64_t GetNextState() const {
    if (payload_.position < payload_.transitions_.size()) {
      return payload_.transitions_[payload_.position].state;
    }

    // else
    return 0;
  }

  unsigned char GetNextTransition() const {
    return payload_.transitions_[payload_.position].label;
  }

  size_t size() const {
    return payload_.transitions_.size();
  }

  size_t& operator++ (){
    return ++payload_.position;
  }

  size_t operator++ (int){
    return payload_.position++;
  }

  void Clear(){
    payload_.position = 0;
    payload_.transitions_.clear();
  }

  void PostProcess(){
  }

  TraversalStatePayload<TransitionT> payload_;
};

/**
 * A helper data structure memorize the path of a graph traversal.
 */
template<class TransitionT = Transition>
struct TraversalStack {
  TraversalStack():traversal_states(), current_depth(0) {
    traversal_states.resize(20);
  }

  TraversalState<TransitionT>& GetStates() {
    return traversal_states[current_depth];
  }

  size_t GetDepth() const {
    return current_depth;
  }

  size_t& operator++ (){
    // resize if needed
    if (traversal_states.size() < current_depth + 2) {
      traversal_states.resize(current_depth + 10);
    }
    return ++current_depth;
  }

  size_t operator++ (int){
    current_depth++;
    // resize if needed
    if (traversal_states.size() < current_depth + 1) {
      traversal_states.resize(current_depth + 10);
    }

    return current_depth;
  }

  size_t& operator-- (){
    return --current_depth;
  }

  size_t operator-- (int){
    return current_depth--;
  }

  std::vector<TraversalState<TransitionT>> traversal_states;
  size_t current_depth;
};


} /* namespace traversal */
} /* namespace fsa */
} /* namespace dictionary */
} /* namespace keyvi */

#endif /* TRAVERSAL_BASE_H_ */
