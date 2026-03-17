import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Provider } from 'react-redux';
import { QueryClient, QueryClientProvider } from 'react-query';
import { store } from './store';
import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import CreditProfile from './pages/CreditProfile';
import Loans from './pages/Loans';
import Reputation from './pages/Reputation';
import Settings from './pages/Settings';
import './App.css';

const queryClient = new QueryClient();

function App() {
  return (
    <Provider store={store}>
      <QueryClientProvider client={queryClient}>
        <Router>
          <Layout>
            <Routes>
              <Route path="/" element={<Dashboard />} />
              <Route path="/dashboard" element={<Dashboard />} />
              <Route path="/credit-profile" element={<CreditProfile />} />
              <Route path="/loans" element={<Loans />} />
              <Route path="/reputation" element={<Reputation />} />
              <Route path="/settings" element={<Settings />} />
            </Routes>
          </Layout>
        </Router>
      </QueryClientProvider>
    </Provider>
  );
}

export default App;
